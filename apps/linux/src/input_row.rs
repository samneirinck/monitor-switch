mod imp {
    use gtk4::glib;
    use gtk4::subclass::prelude::*;
    use gtk4::{CompositeTemplate, Image, Label, TemplateChild};
    use std::cell::Cell;

    #[derive(Default, CompositeTemplate)]
    #[template(resource = "/com/github/samneirinck/MonitorSwitch/input_row.ui")]
    pub struct MonitorSwitchInputRow {
        #[template_child]
        pub label: TemplateChild<Label>,
        #[template_child]
        pub check_icon: TemplateChild<Image>,

        pub monitor_index: Cell<usize>,
        pub input_value: Cell<u8>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MonitorSwitchInputRow {
        const NAME: &'static str = "MonitorSwitchInputRow";
        type Type = super::MonitorSwitchInputRow;
        type ParentType = gtk4::ListBoxRow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for MonitorSwitchInputRow {}
    impl WidgetImpl for MonitorSwitchInputRow {}
    impl ListBoxRowImpl for MonitorSwitchInputRow {}
}

use gtk4::glib;
use gtk4::subclass::prelude::ObjectSubclassIsExt;
use monitor_core::InputSource;

glib::wrapper! {
    pub struct MonitorSwitchInputRow(ObjectSubclass<imp::MonitorSwitchInputRow>)
        @extends gtk4::ListBoxRow, gtk4::Widget,
        @implements gtk4::Accessible, gtk4::Actionable, gtk4::Buildable, gtk4::ConstraintTarget;
}

impl MonitorSwitchInputRow {
    pub fn new(label: &str, is_current: bool, monitor_index: usize, input: InputSource) -> Self {
        let row: Self = glib::Object::builder().build();
        let imp = row.imp();

        imp.label.set_label(label);
        imp.check_icon.set_visible(is_current);
        imp.monitor_index.set(monitor_index);
        imp.input_value.set(input.to_vcp_value());

        row
    }

    pub fn monitor_index(&self) -> usize {
        self.imp().monitor_index.get()
    }

    pub fn input(&self) -> InputSource {
        InputSource::from_vcp_value(self.imp().input_value.get())
    }
}

