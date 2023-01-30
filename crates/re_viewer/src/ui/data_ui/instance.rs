use re_data_store::{EntityPath, Index, InstanceId};
use re_log_types::ComponentPath;
use re_query::{get_component_with_instances, QueryError};

use crate::{
    misc::ViewerContext,
    ui::{format_component_name, UiVerbosity},
};

use super::{
    component::{arrow_component_elem_ui, arrow_component_ui},
    DataUi,
};

impl DataUi for EntityPath {
    fn data_ui(
        &self,
        ctx: &mut ViewerContext<'_>,
        ui: &mut egui::Ui,
        verbosity: UiVerbosity,
        query: &re_arrow_store::LatestAtQuery,
    ) {
        InstanceId {
            entity_path: self.clone(),
            instance_index: None,
        }
        .data_ui(ctx, ui, verbosity, query);
    }
}

impl DataUi for InstanceId {
    fn data_ui(
        &self,
        ctx: &mut ViewerContext<'_>,
        ui: &mut egui::Ui,
        _verbosity: UiVerbosity,
        query: &re_arrow_store::LatestAtQuery,
    ) {
        let store = &ctx.log_db.entity_db.arrow_store;

        let Some(mut components) = store.all_components(&query.timeline, &self.entity_path) else {
            ui.label("No Components");
            return ;
        };
        components.sort();

        egui::Grid::new("entity_instance")
            .num_columns(2)
            .show(ui, |ui| {
                for component_name in components {
                    let component_data = get_component_with_instances(
                        store,
                        query,
                        &self.entity_path,
                        component_name,
                    );

                    if matches!(component_data, Err(QueryError::PrimaryNotFound)) {
                        continue; // no need to show components that are unset
                    }

                    ctx.component_path_button_to(
                        ui,
                        format_component_name(&component_name),
                        &ComponentPath::new(self.entity_path.clone(), component_name),
                    );

                    match (component_data, &self.instance_index) {
                        // If we didn't find the component then it's not set at this point in time
                        (Err(QueryError::PrimaryNotFound), _) => {
                            ui.label("<unset>");
                        }
                        // Any other failure to get a component is unexpected
                        (Err(err), _) => {
                            ui.label(format!("Error: {}", err));
                        }
                        // If an `instance_index` wasn't provided, just report the number of values
                        (Ok(component_data), None) => {
                            arrow_component_ui(ctx, ui, &component_data, UiVerbosity::Small, query);
                        }
                        // If the `instance_index` is an `ArrowInstance` show the value
                        (Ok(component_data), Some(Index::ArrowInstance(instance))) => {
                            arrow_component_elem_ui(
                                ctx,
                                ui,
                                UiVerbosity::Small,
                                query,
                                &component_data,
                                instance,
                            );
                        }
                        // If the `instance_index` isn't an `ArrowInstance` something has gone wrong
                        // TODO(jleibs) this goes away once all indexes are just `Instances`
                        (Ok(_), Some(_)) => {
                            ui.label("<bad index>");
                        }
                    };

                    ui.end_row();
                }
                Some(())
            });
    }
}