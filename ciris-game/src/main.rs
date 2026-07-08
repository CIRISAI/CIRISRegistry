use macroquad::prelude::*;
mod ceg;
use ceg::{CegEngine, CohortScope, Envelope, Primitive};
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq)]
enum NodeType {
    Standard,
    Firewall,
    LoadBalancer,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum BallType {
    Update,
    Hack,
}

struct Org {
    id: usize,
    color: Color,
    center: Vec3,
}

struct Node {
    id: usize,
    org_id: usize,
    pos: Vec3,
    node_type: NodeType,
    active: bool,
}

struct Tube {
    id: usize,
    from_node: usize,
    to_node: usize,
    active: bool,
}

struct Ball {
    id: usize,
    org_id: usize,
    ball_type: BallType,
    from_node: usize,
    to_node: usize,
    progress: f32, // 0.0 to 1.0 along the tube
    env_id: String,
}

struct GameState {
    users: i64,
    orgs: HashMap<usize, Org>,
    nodes: HashMap<usize, Node>,
    tubes: Vec<Tube>,
    balls: Vec<Ball>,
    engine: CegEngine,
    next_node_id: usize,
    next_ball_id: usize,
    next_tube_id: usize,
    player_org_id: usize,
    camera_distance: f32,
    trust_filter_active: bool,
    env_counter: usize,
    camera_angle: f32,
}

impl GameState {
    fn new() -> Self {
        let mut state = Self {
            users: 100,
            orgs: HashMap::new(),
            nodes: HashMap::new(),
            tubes: Vec::new(),
            balls: Vec::new(),
            engine: CegEngine::new(),
            next_node_id: 0,
            next_ball_id: 0,
            next_tube_id: 0,
            player_org_id: 0,
            camera_distance: 20.0,
            trust_filter_active: false,
            env_counter: 0,
            camera_angle: 0.0,
        };
        state.init_world();
        state
    }

    fn init_world(&mut self) {
        // Player Org
        self.orgs.insert(0, Org { id: 0, color: BLUE, center: Vec3::ZERO });
        self.add_node(0, Vec3::new(0.0, 0.0, 0.0), NodeType::Standard);

        // External Orgs
        let colors = [RED, GREEN, YELLOW, PURPLE];
        for i in 1..=4 {
            let angle = (i as f32) * std::f32::consts::PI / 2.0;
            let dist = 15.0;
            let center = Vec3::new(angle.cos() * dist, 0.0, angle.sin() * dist);
            self.orgs.insert(i, Org { id: i, color: colors[i-1], center });

            let ext_node = self.add_node(i, center, NodeType::Standard);
            // Connect to player core
            let tube_id = self.next_tube_id;
            self.next_tube_id += 1;
            self.tubes.push(Tube { id: tube_id, from_node: ext_node, to_node: 0, active: true });
        }
    }

    fn add_node(&mut self, org_id: usize, pos: Vec3, node_type: NodeType) -> usize {
        let id = self.next_node_id;
        self.next_node_id += 1;
        self.nodes.insert(id, Node { id, org_id, pos, node_type, active: true });
        id
    }

    fn spawn_ball(&mut self, org_id: usize) {
        let org_node = self.nodes.values().find(|n| n.org_id == org_id).unwrap().id;

        // Find a random active player node that is connected
        let mut possible_targets = Vec::new();
        for tube in &self.tubes {
            if tube.active && tube.from_node == org_node {
                if let Some(target) = self.nodes.get(&tube.to_node) {
                    if target.active {
                        possible_targets.push(target.id);
                    }
                }
            }
        }

        if possible_targets.is_empty() { return; }

        let player_node = possible_targets[rand::gen_range(0, possible_targets.len())];

        let is_hack = rand::gen_range(0, 100) < 30;
        let env_id = format!("packet_{}", self.env_counter);
        self.env_counter += 1;

        self.balls.push(Ball {
            id: self.next_ball_id,
            org_id,
            ball_type: if is_hack { BallType::Hack } else { BallType::Update },
            from_node: org_node,
            to_node: player_node,
            progress: 0.0,
            env_id: env_id.clone(),
        });
        self.next_ball_id += 1;

        // Emit underlying CEG
        self.engine.emit(Envelope {
            id: env_id,
            attesting_key_id: format!("org_{}", org_id),
            cohort_scope: CohortScope::Community,
            primitive: Primitive::Scores {
                dimension: if is_hack { "infohazard".to_string() } else { "good_action".to_string() },
                score: 1.0,
                confidence: 1.0,
            },
        });
    }

    fn apply_firewall(&mut self, tube_id: usize) {
        if let Some(tube) = self.tubes.iter_mut().find(|t| t.id == tube_id) {
            tube.active = false;
        }
    }
}

// Function to find closest point on line segment
fn closest_point_on_segment(p: Vec3, a: Vec3, b: Vec3) -> Vec3 {
    let ap = p - a;
    let ab = b - a;
    let ab2 = ab.x*ab.x + ab.y*ab.y + ab.z*ab.z;
    let ap_ab = ap.x*ab.x + ap.y*ab.y + ap.z*ab.z;
    let mut t = ap_ab / ab2;
    if t < 0.0 { t = 0.0; }
    else if t > 1.0 { t = 1.0; }
    a + ab * t
}

#[macroquad::main("CIRIS Constitution Explorer 3D")]
async fn main() {
    let mut state = GameState::new();
    let mut last_time = get_time();

    loop {
        let dt = (get_time() - last_time) as f32;
        last_time = get_time();

        clear_background(color_u8!(10, 10, 15, 255));

        let max_users: i64 = 8_000_000_000;
        if state.users > max_users {
            state.users = max_users;
        }

        state.camera_angle += 0.1 * dt;

        let scale_factor = (state.users as f32 / 100.0).max(1.0).log2();
        state.camera_distance = 20.0 + scale_factor * 8.0;

        let cam_pos = vec3(
            state.camera_angle.cos() * state.camera_distance,
            state.camera_distance * 0.8,
            state.camera_angle.sin() * state.camera_distance,
        );
        let cam_target = vec3(0.0, 0.0, 0.0);
        let cam_up = vec3(0.0, 1.0, 0.0);

        let required_player_nodes = (state.users / 10).max(1) as usize;
        let current_player_nodes = state.nodes.values().filter(|n| n.org_id == 0 && n.active).count();

        if current_player_nodes < required_player_nodes {
            let angle = rand::gen_range(0.0, std::f32::consts::PI * 2.0);
            let dist = rand::gen_range(2.0, 5.0 + scale_factor);
            let pos = Vec3::new(angle.cos() * dist, rand::gen_range(-2.0, 2.0), angle.sin() * dist);
            let new_node = state.add_node(0, pos, NodeType::Standard);

            let tube_id = state.next_tube_id;
            state.next_tube_id += 1;
            state.tubes.push(Tube { id: tube_id, from_node: new_node, to_node: 0, active: true });

            // Connect this new node to a random external org to increase packet flow
            let rand_org = rand::gen_range(1, 5);
            if let Some(ext_node) = state.nodes.values().find(|n| n.org_id == rand_org) {
                let tube_id2 = state.next_tube_id;
                state.next_tube_id += 1;
                state.tubes.push(Tube { id: tube_id2, from_node: ext_node.id, to_node: new_node, active: true });
            }
        }

        if rand::gen_range(0, 100) < 5 + scale_factor as i32 {
            let rand_org = rand::gen_range(1, 5);
            state.spawn_ball(rand_org);
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_pos = mouse_position();
            let filter_rect = Rect::new(screen_width() - 220.0, 20.0, 200.0, 40.0);
            if filter_rect.contains(mouse_pos.into()) {
                state.trust_filter_active = !state.trust_filter_active;
            } else {
                // Raycast to sever tubes (Firewall / Slashing)
                // Simplified clicking: since we lack complex 3D raycasting easily in macroquad 3D right now,
                // we'll randomly slash a tube if we click on the left side of the screen as a demo mechanic
                if mouse_pos.0 < 200.0 {
                    // find a random active tube from external to internal
                    for tube in state.tubes.iter_mut() {
                        if tube.active && tube.to_node != 0 {
                            tube.active = false;
                            break;
                        }
                    }
                }
            }
        }

        // Update Balls
        let mut balls_to_remove = Vec::new();
        for i in 0..state.balls.len() {
            let ball = &mut state.balls[i];

            // Check if tube is severed
            let mut tube_active = false;
            for tube in &state.tubes {
                if tube.from_node == ball.from_node && tube.to_node == ball.to_node && tube.active {
                    tube_active = true;
                }
            }

            if !tube_active {
                // Ball drops/dies
                balls_to_remove.push(i);
                continue;
            }

            ball.progress += 0.2 * dt;

            if ball.progress >= 1.0 {
                balls_to_remove.push(i);

                match ball.ball_type {
                    BallType::Update => {
                        state.users += 5;
                    },
                    BallType::Hack => {
                        if !state.trust_filter_active {
                            state.users -= 10;
                            if state.users < 0 {
                                state.users = 0;
                            }
                        } else {
                            state.engine.emit(Envelope {
                                id: format!("slash_{}", state.env_counter),
                                attesting_key_id: "agent_node".to_string(),
                                cohort_scope: CohortScope::Affiliation,
                                primitive: Primitive::Withdraws { target_id: ball.env_id.clone() },
                            });
                            state.env_counter += 1;
                        }
                    }
                }
            }
        }
        for idx in balls_to_remove.iter().rev() {
            state.balls.remove(*idx);
        }

        // 3D Rendering
        set_camera(&Camera3D {
            position: cam_pos,
            up: cam_up,
            target: cam_target,
            ..Default::default()
        });

        draw_grid(20, 1.0, DARKGRAY, GRAY);

        for tube in &state.tubes {
            if !tube.active { continue; }
            if let (Some(n1), Some(n2)) = (state.nodes.get(&tube.from_node), state.nodes.get(&tube.to_node)) {
                if n1.active && n2.active {
                    draw_line_3d(n1.pos, n2.pos, color_u8!(100, 100, 100, 100));
                }
            }
        }

        for ball in &state.balls {
            if let (Some(n1), Some(n2)) = (state.nodes.get(&ball.from_node), state.nodes.get(&ball.to_node)) {
                let current_pos = n1.pos.lerp(n2.pos, ball.progress);
                let color = match ball.ball_type {
                    BallType::Update => GREEN,
                    BallType::Hack => RED,
                };
                draw_sphere(current_pos, 0.3, None, color);
            }
        }

        for node in state.nodes.values() {
            if node.active {
                let color = state.orgs.get(&node.org_id).unwrap().color;
                let radius = match node.node_type {
                    NodeType::Standard => 0.5,
                    NodeType::Firewall => 0.8,
                    NodeType::LoadBalancer => 0.7,
                };
                draw_sphere(node.pos, radius, None, color);
            }
        }

        // Back to 2D for UI
        set_default_camera();

        draw_text(&format!("Users: {}", state.users), 20.0, 40.0, 30.0, WHITE);

        let filter_rect = Rect::new(screen_width() - 220.0, 20.0, 200.0, 40.0);
        draw_rectangle(filter_rect.x, filter_rect.y, filter_rect.w, filter_rect.h, if state.trust_filter_active { BLUE } else { DARKGRAY });
        draw_text("Toggle Trust Filter", filter_rect.x + 10.0, filter_rect.y + 25.0, 18.0, WHITE);

        draw_text("Click Left Screen to SLASH active external tubes", 20.0, screen_height() - 30.0, 20.0, LIGHTGRAY);

        if state.users <= 0 {
            draw_text("NETWORK COLLAPSE", screen_width() / 2.0 - 150.0, screen_height() / 2.0, 40.0, RED);
        }

        next_frame().await;
    }
}
