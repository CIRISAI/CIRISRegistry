# AWS archive — stale ECS tooling (NOT active)

This directory holds deployment tooling for an AWS ECS task-definition
deployment that CIRISRegistry **no longer runs**. Current prod is GHCR +
watchtower on Vultr/Hetzner (US + EU).

- `rollback.yml` — formerly `deploy/ansible/playbooks/rollback.yml`. Uses
  `amazon.aws.ecs_service_info` / `ecs_task_definition_info`; will error against
  the current infra. Archived per CIRISRegistry#42 so it stops misleading
  operators during incidents.

For the live rollback process see [`RELEASE.md`](../../RELEASE.md) "Rollback".
