use crate::*;
use egui_inspect::EguiInspect;

impl<T, const N: usize> EguiInspect for vec<T, N>
where
	T: EguiInspect,
{
	fn inspect(&self, label: &str, ui: &mut egui::Ui) {
		ui.horizontal(|ui| {
			ui.label(label);
			for i in 0..N {
				self[i].inspect("", ui);
			}
		});
	}
	fn inspect_mut(&mut self, label: &str, ui: &mut egui::Ui) {
		ui.horizontal(|ui| {
			ui.label(label);
			for i in 0..N {
				self[i].inspect_mut("", ui);
			}
		});
	}
}
