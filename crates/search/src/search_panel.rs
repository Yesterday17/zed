use gpui::{
    actions, prelude::*, Action, AppContext, AsyncWindowContext, EventEmitter, FocusHandle,
    FocusableView, View, WeakView,
};
use ui::prelude::*;
use workspace::{
    dock::{Panel, PanelEvent},
    Workspace,
};

actions!(search_panel, [ToggleFocus]);

pub struct SearchPanel {
    focus_handle: FocusHandle,
}

impl SearchPanel {
    pub fn new(workspace: &mut Workspace, cx: &mut ViewContext<Workspace>) -> View<Self> {
        cx.new_view(|cx| Self {
            focus_handle: cx.focus_handle(),
        })
    }

    pub async fn load(
        workspace: WeakView<Workspace>,
        mut cx: AsyncWindowContext,
    ) -> anyhow::Result<View<Self>> {
        workspace.update(&mut cx, |workspace, cx| {
            let panel = SearchPanel::new(workspace, cx);
            panel
        })
    }
}

impl Render for SearchPanel {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
    }
}

impl Panel for SearchPanel {
    fn persistent_name() -> &'static str {
        "Project Search Panel"
    }

    fn position(&self, cx: &WindowContext) -> workspace::dock::DockPosition {
        workspace::dock::DockPosition::Left
    }

    fn position_is_valid(&self, position: workspace::dock::DockPosition) -> bool {
        true
    }

    fn set_position(
        &mut self,
        position: workspace::dock::DockPosition,
        cx: &mut ViewContext<Self>,
    ) {
    }

    fn size(&self, cx: &WindowContext) -> Pixels {
        Pixels(240.)
    }

    fn set_size(&mut self, size: Option<Pixels>, cx: &mut ViewContext<Self>) {}

    fn icon(&self, cx: &WindowContext) -> Option<IconName> {
        Some(IconName::MagnifyingGlass)
    }

    fn icon_tooltip(&self, cx: &WindowContext) -> Option<&'static str> {
        Some("Search")
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(ToggleFocus)
    }
}

impl EventEmitter<PanelEvent> for SearchPanel {}

impl FocusableView for SearchPanel {
    fn focus_handle(&self, cx: &AppContext) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}
