use system_tray::{
    icon::Icon,
    menu::{
        menu_event_receiver, AboutMetadata, Menu, MenuEventReceiver, MenuItem, PredefinedMenuItem,
    },
    tray_event_receiver, TrayEventReceiver, TrayIcon, TrayIconBuilder,
};

pub enum MenuAction {
    Exit,
}

pub struct Tray<'a> {
    tray_icon: Option<TrayIcon>,
    exit_menu_item: MenuItem,
    menu_event_receiver: &'a MenuEventReceiver,
    tray_event_receiver: &'a TrayEventReceiver,
}

impl<'a> Tray<'a> {
    pub fn new() -> Self {
        let tray_menu = Menu::new();
        let exit_menu_item = MenuItem::new("Exit", true, None);
        tray_menu.append_items(&[
            &PredefinedMenuItem::about(
                None,
                Some(AboutMetadata {
                    name: Some("TotalMix Volume Control".to_string()),
                    version: Some("1.0.0".to_string()),
                    authors: Some(vec!["Fotis Gimian".to_string()]),
                    license: Some("MIT or Apache 2.0".to_string()),
                    website: Some("https://github.com/fgimian/totalmix-volume-control".to_string()),
                    ..Default::default()
                }),
            ),
            &PredefinedMenuItem::separator(),
            &exit_menu_item,
        ]);

        let icon = Icon::from_resource(1, None).unwrap();
        let tray_icon = TrayIconBuilder::new()
            .with_menu(Box::new(tray_menu))
            .with_tooltip("TotalMix OSC connection active")
            .with_icon(icon)
            .build()
            .unwrap();

        Self {
            tray_icon: Some(tray_icon),
            exit_menu_item,
            menu_event_receiver: menu_event_receiver(),
            tray_event_receiver: tray_event_receiver(),
        }
    }

    pub fn receive_menu_event(&self) -> Option<MenuAction> {
        self.menu_event_receiver
            .try_recv()
            .map_or(None, |menu_event| {
                if menu_event.id == self.exit_menu_item.id() {
                    Some(MenuAction::Exit)
                } else {
                    None
                }
            })
    }

    pub fn receive_tray_event(&self) {
        if let Ok(_tray_event) = self.tray_event_receiver.try_recv() {}
    }
}

impl<'a> Drop for Tray<'a> {
    fn drop(&mut self) {
        self.tray_icon.take();
    }
}
