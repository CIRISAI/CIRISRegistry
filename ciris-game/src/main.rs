use macroquad::prelude::*;
mod ceg;
use ceg::{CegEngine, CohortScope, Envelope, Primitive};

struct Node {
    pos: Vec2,
    color: Color,
    radius: f32,
    is_malicious: bool,
    active: bool,
    envelope_id: String,
}

#[macroquad::main("CIRIS Constitution Explorer")]
async fn main() {
    let mut current_objective = 0; // 0 = menu, 1, 2, 3 = objectives
    let mut engine = CegEngine::new();

    // Obj 1 state
    let mut current_tier = CohortScope::SelfScope;
    let mut tier_progress = 0.0;

    // Obj 2 state
    let mut packets: Vec<Node> = Vec::new();
    let mut filter_active = false;
    let mut packet_id_counter = 0;

    // Obj 3 state
    let mut holders = vec![
        Vec2::new(100.0, 100.0),
        Vec2::new(700.0, 150.0),
        Vec2::new(400.0, 500.0)
    ];
    let mut selected_holder: Option<usize> = None;
    let mut shutdown_triggered = false;

    let mut last_time = get_time();

    loop {
        let dt = (get_time() - last_time) as f32;
        last_time = get_time();

        clear_background(color_u8!(20, 20, 30, 255));

        if current_objective == 0 {
            draw_menu(&mut current_objective);
        } else if current_objective == 1 {
            update_objective_1(&mut current_tier, &mut tier_progress, &mut engine);
            draw_objective_1(&current_tier, tier_progress, &mut current_objective);
        } else if current_objective == 2 {
            update_objective_2(&mut packets, &mut engine, &mut filter_active, &mut packet_id_counter, dt);
            draw_objective_2(&packets, &engine, filter_active, &mut current_objective);
        } else if current_objective == 3 {
            update_objective_3(&mut holders, &mut selected_holder, &mut shutdown_triggered);
            draw_objective_3(&holders, shutdown_triggered, &mut current_objective);
        }

        next_frame().await;
    }
}

fn draw_menu(current_objective: &mut i32) {
    draw_text("CIRIS Constitution Explorer", 50.0, 50.0, 40.0, WHITE);

    let w = screen_width();

    let btn_w = 400.0;
    let btn_h = 50.0;
    let btn_x = (w - btn_w) / 2.0;

    let obj1_rect = Rect::new(btn_x, 150.0, btn_w, btn_h);
    let obj2_rect = Rect::new(btn_x, 220.0, btn_w, btn_h);
    let obj3_rect = Rect::new(btn_x, 290.0, btn_w, btn_h);

    draw_rectangle(obj1_rect.x, obj1_rect.y, obj1_rect.w, obj1_rect.h, BLUE);
    draw_text("1. Scope Expansion", obj1_rect.x + 10.0, obj1_rect.y + 32.0, 30.0, WHITE);

    draw_rectangle(obj2_rect.x, obj2_rect.y, obj2_rect.w, obj2_rect.h, RED);
    draw_text("2. The Moderation Ratchet", obj2_rect.x + 10.0, obj2_rect.y + 32.0, 30.0, WHITE);

    draw_rectangle(obj3_rect.x, obj3_rect.y, obj3_rect.w, obj3_rect.h, GREEN);
    draw_text("3. The Humanity Accord", obj3_rect.x + 10.0, obj3_rect.y + 32.0, 30.0, WHITE);

    if is_mouse_button_pressed(MouseButton::Left) {
        let mouse_pos = mouse_position();
        if obj1_rect.contains(mouse_pos.into()) {
            *current_objective = 1;
        } else if obj2_rect.contains(mouse_pos.into()) {
            *current_objective = 2;
        } else if obj3_rect.contains(mouse_pos.into()) {
            *current_objective = 3;
        }
    }
}

fn update_objective_1(tier: &mut CohortScope, progress: &mut f32, engine: &mut CegEngine) {
    let btn_rect = Rect::new(screen_width() / 2.0 - 100.0, 300.0, 200.0, 50.0);
    if is_mouse_button_pressed(MouseButton::Left) {
        let mouse_pos = mouse_position();
        if btn_rect.contains(mouse_pos.into()) {
            engine.emit(Envelope {
                id: format!("action_{}", rand::rand()),
                attesting_key_id: "user1".to_string(),
                cohort_scope: tier.clone(),
                primitive: Primitive::Scores { dimension: "activity".to_string(), score: 1.0, confidence: 1.0 },
            });

            *progress += 20.0;
            if *progress >= 100.0 {
                *progress = 0.0;
                *tier = match tier {
                    CohortScope::SelfScope => CohortScope::Family,
                    CohortScope::Family => CohortScope::Community,
                    CohortScope::Community => CohortScope::Affiliation,
                    CohortScope::Affiliation => CohortScope::Affiliation,
                };
            }
        }
    }
}

fn draw_objective_1(tier: &CohortScope, progress: f32, current_objective: &mut i32) {
    draw_text("Objective 1: Scope Expansion", 20.0, 40.0, 30.0, BLUE);

    let tier_str = match tier {
        CohortScope::SelfScope => "Self (Local Occurrences only)",
        CohortScope::Family => "Family (Invisible Encrypted Routing)",
        CohortScope::Community => "Community (Provenance-Visible)",
        CohortScope::Affiliation => "Affiliation (Institutional, Governance)",
    };

    draw_text(&format!("Current Tier: {}", tier_str), 20.0, 100.0, 24.0, WHITE);

    let cx = screen_width() / 2.0;

    draw_circle(cx, 200.0, 50.0 + progress * 0.5, match tier {
        CohortScope::SelfScope => GRAY,
        CohortScope::Family => ORANGE,
        CohortScope::Community => GREEN,
        CohortScope::Affiliation => PURPLE,
    });

    let btn_rect = Rect::new(cx - 100.0, 300.0, 200.0, 50.0);
    draw_rectangle(btn_rect.x, btn_rect.y, btn_rect.w, btn_rect.h, BLUE);
    draw_text("Generate Occurrences", btn_rect.x + 10.0, btn_rect.y + 30.0, 18.0, WHITE);

    draw_rectangle(cx - 100.0, 370.0, 200.0, 20.0, DARKGRAY);
    draw_rectangle(cx - 100.0, 370.0, progress * 2.0, 20.0, YELLOW);

    draw_text("Unlocked Features:", 20.0, 420.0, 20.0, WHITE);
    draw_text("- Local Data", 40.0, 450.0, 18.0, LIGHTGRAY);
    if *tier != CohortScope::SelfScope {
        draw_text("- Encrypted File Sharing", 40.0, 470.0, 18.0, ORANGE);
    }
    if *tier == CohortScope::Community || *tier == CohortScope::Affiliation {
        draw_text("- Social & Moderation", 40.0, 490.0, 18.0, GREEN);
    }
    if *tier == CohortScope::Affiliation {
        draw_text("- Institutional Compliance", 40.0, 510.0, 18.0, PURPLE);
    }

    draw_back_button(current_objective);
}

fn update_objective_2(packets: &mut Vec<Node>, engine: &mut CegEngine, filter_active: &mut bool, counter: &mut u32, dt: f32) {
    let filter_rect = Rect::new(screen_width() - 220.0, 20.0, 200.0, 40.0);
    if is_mouse_button_pressed(MouseButton::Left) {
        let mouse_pos = mouse_position();
        if filter_rect.contains(mouse_pos.into()) {
            *filter_active = !*filter_active;
            // Update delegates_to
            if *filter_active {
                engine.emit(Envelope {
                    id: format!("filter_{}", counter),
                    attesting_key_id: "agent_node".to_string(),
                    cohort_scope: CohortScope::Affiliation,
                    primitive: Primitive::DelegatesTo { delegated_scope: vec!["moderate".to_string()] },
                });
            } else {
                engine.emit(Envelope {
                    id: format!("filter_off_{}", counter),
                    attesting_key_id: "agent_node".to_string(),
                    cohort_scope: CohortScope::Affiliation,
                    primitive: Primitive::Withdraws { target_id: format!("filter_{}", *counter - 1) },
                });
            }
            *counter += 1;
        }
    }

    if is_mouse_button_pressed(MouseButton::Left) {
        let mouse_pos: Vec2 = mouse_position().into();
        for p in packets.iter_mut() {
            if p.pos.distance(mouse_pos) < p.radius {
                p.active = false;
                // Emit CEG withdrawal for slashed packet
                engine.emit(Envelope {
                    id: format!("slash_{}", counter),
                    attesting_key_id: "user1".to_string(),
                    cohort_scope: CohortScope::Affiliation,
                    primitive: Primitive::Withdraws { target_id: p.envelope_id.clone() },
                });
                *counter += 1;
            }
        }
    }

    if rand::gen_range(0, 100) < 5 {
        let is_mal = rand::gen_range(0, 100) < 30;
        let env_id = format!("packet_{}", counter);
        *counter += 1;

        packets.push(Node {
            pos: Vec2::new(rand::gen_range(50.0, screen_width() - 50.0), -20.0),
            color: if is_mal { RED } else { GREEN },
            radius: 15.0,
            is_malicious: is_mal,
            active: true,
            envelope_id: env_id.clone(),
        });

        // Emit underlying CEG record when generated
        engine.emit(Envelope {
            id: env_id,
            attesting_key_id: "unknown".to_string(),
            cohort_scope: CohortScope::Community,
            primitive: Primitive::Scores {
                dimension: if is_mal { "infohazard".to_string() } else { "good_action".to_string() },
                score: 1.0,
                confidence: 1.0,
            },
        });
    }

    let cx = screen_width() / 2.0;
    let cy = screen_height() - 100.0;
    let core_pos = Vec2::new(cx, cy);

    for p in packets.iter_mut() {
        if !p.active { continue; }
        let dir = (core_pos - p.pos).normalize();
        p.pos += dir * 100.0 * dt;

        if p.pos.distance(core_pos) < 50.0 {
            p.active = false;
            // If malicious and filter is ON, we slash it automatically before it hits coherence
            if p.is_malicious && *filter_active {
                engine.emit(Envelope {
                    id: format!("slash_{}", counter),
                    attesting_key_id: "agent_node".to_string(),
                    cohort_scope: CohortScope::Affiliation,
                    primitive: Primitive::Withdraws { target_id: p.envelope_id.clone() },
                });
                *counter += 1;
            }
        }
    }

    packets.retain(|p| p.active);
}

fn draw_objective_2(packets: &Vec<Node>, engine: &CegEngine, filter_active: bool, current_objective: &mut i32) {
    draw_text("Objective 2: The Moderation Ratchet", 20.0, 40.0, 30.0, RED);

    let coherence = engine.get_coherence_score();
    draw_text(&format!("Coherence: {:.0}%", coherence), 20.0, 80.0, 24.0, if coherence > 50.0 { GREEN } else { RED });

    let filter_rect = Rect::new(screen_width() - 220.0, 20.0, 200.0, 40.0);
    draw_rectangle(filter_rect.x, filter_rect.y, filter_rect.w, filter_rect.h, if filter_active { BLUE } else { DARKGRAY });
    draw_text("Toggle Trust Filter", filter_rect.x + 10.0, filter_rect.y + 25.0, 18.0, WHITE);

    draw_text("Click RED infohazards to slash them!", 20.0, 110.0, 18.0, LIGHTGRAY);

    let cx = screen_width() / 2.0;
    let cy = screen_height() - 100.0;

    draw_circle(cx, cy, 50.0, BLUE);
    draw_circle_lines(cx, cy, 70.0, 2.0, if filter_active { SKYBLUE } else { BLANK });

    for p in packets {
        if p.active {
            draw_circle(p.pos.x, p.pos.y, p.radius, p.color);
        }
    }

    if coherence <= 0.0 {
        draw_text("COHERENCE LOST. SHUTDOWN.", cx - 150.0, cy - 80.0, 30.0, RED);
    }

    draw_back_button(current_objective);
}

fn update_objective_3(holders: &mut Vec<Vec2>, selected: &mut Option<usize>, triggered: &mut bool) {
    let mouse_pos: Vec2 = mouse_position().into();

    if is_mouse_button_pressed(MouseButton::Left) {
        *selected = None;
        for (i, h) in holders.iter().enumerate() {
            if h.distance(mouse_pos) < 30.0 {
                *selected = Some(i);
                break;
            }
        }
    }

    if is_mouse_button_down(MouseButton::Left) {
        if let Some(idx) = *selected {
            holders[idx] = mouse_pos;
        }
    }

    let cx = screen_width() / 2.0;
    let cy = screen_height() / 2.0;
    let core = Vec2::new(cx, cy);

    let mut in_core = 0;
    for h in holders.iter() {
        if h.distance(core) < 60.0 {
            in_core += 1;
        }
    }

    if in_core >= 2 {
        *triggered = true;
    } else {
        *triggered = false;
    }
}

fn draw_objective_3(holders: &Vec<Vec2>, triggered: bool, current_objective: &mut i32) {
    draw_text("Objective 3: The Humanity Accord", 20.0, 40.0, 30.0, GREEN);
    draw_text("Drag 2 of 3 Accord Holders to the Core to trigger CONSTITUTIONAL halt.", 20.0, 70.0, 18.0, WHITE);

    let cx = screen_width() / 2.0;
    let cy = screen_height() / 2.0;

    draw_circle(cx, cy, 60.0, if triggered { RED } else { DARKGRAY });
    draw_text("CORE", cx - 25.0, cy + 5.0, 20.0, WHITE);

    for (i, h) in holders.iter().enumerate() {
        draw_circle(h.x, h.y, 30.0, YELLOW);
        draw_text(&format!("H{}", i+1), h.x - 12.0, h.y + 6.0, 20.0, BLACK);
    }

    if triggered {
        draw_text("CONSTITUTIONAL HALT INITIATED! 2-of-3 Multi-sig verified.", 20.0, 120.0, 24.0, RED);
    }

    draw_back_button(current_objective);
}

fn draw_back_button(current_objective: &mut i32) {
    let back_rect = Rect::new(20.0, screen_height() - 60.0, 100.0, 40.0);
    draw_rectangle(back_rect.x, back_rect.y, back_rect.w, back_rect.h, DARKGRAY);
    draw_text("Back", back_rect.x + 20.0, back_rect.y + 25.0, 20.0, WHITE);

    if is_mouse_button_pressed(MouseButton::Left) {
        let mouse_pos = mouse_position();
        if back_rect.contains(mouse_pos.into()) {
            *current_objective = 0;
        }
    }
}
