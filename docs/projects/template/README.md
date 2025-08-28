# Project Templates

Generic templates for creating new projects (features, demos, polishing tasks, etc.) using the established documentation and planning structure.

## Using the Templates

### For New Projects

1. **Copy template files** to your new project directory:
   ```
   docs/projects/your-project-name/
   ├── spec-template.md → your-project-spec.md
   └── plan-template.md → plan.md
   ```

2. **Replace placeholders** in both files:
   - `[PROJECT_NAME]` → Your project name
   - `[Brief description...]` → Your specific descriptions
   - `[X-Y] hours` → Your time estimates

3. **Customize sections** based on your project type:
   - Add/remove phases in the implementation plan
   - Adjust scope and components in the specification
   - Update approach details for your specific requirements

### Template Structure

**`spec-template.md`** provides the basic specification structure:
- Project overview and purpose
- Implementation scope and components
- Success criteria and requirements

**`plan-template.md`** provides the implementation planning structure:
- Phase-based development approach
- Task breakdown with dependencies
- Discrete, committable checkpoints

### Adaptation Guidelines

- Adapt the scope and components sections based on your project type (feature, demo, polishing, etc.)
- Maintain the phase-based planning approach for systematic development
- Use checkboxes `[ ]` for trackable tasks in implementation plans
- Include clear "Definition of Done" criteria for each task

These templates provide just enough structure for systematic project development without being overly prescriptive.