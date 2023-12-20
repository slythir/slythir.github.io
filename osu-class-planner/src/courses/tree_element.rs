use web_sys::{MouseEvent, window};

const USER_TREE_ID: &str = "user-tree";
const USER_TREE_WRAPPER_ID: &str = "user-tree-wrapper";

pub struct TreeElement {
	elem_x: i32,
	elem_y: i32,
	grab_x: i32,
	grab_y: i32,
	dragging: bool,
}

impl TreeElement {

	pub fn new() -> Self {
		Self {
			elem_x: 0,
			elem_y: 0,
			grab_x: 0,
			grab_y: 0,
			dragging: false
		}
	}

	pub fn on_mouse_down(&mut self, event: MouseEvent) {
		self.grab_x = event.client_x();
		self.grab_y = event.client_y();
		self.dragging = true;
	}

	pub fn on_mouse_up(&mut self, event: MouseEvent) {
		if self.dragging {
			(self.elem_x, self.elem_y) = self.get_elem_pos(event);
			self.dragging = false;
		}
	}

	// TODO: keep the grid settings in the style update
	pub fn on_mouse_move(&mut self, event: MouseEvent) {
		if self.dragging {

			let elem_pos = self.get_elem_pos(event);
			let transform = format!("transform: translate({}px, {}px)", elem_pos.0, elem_pos.1);

			let elem = window().unwrap().document().unwrap().get_element_by_id(USER_TREE_ID).unwrap();
			elem.set_attribute("style", &transform).unwrap();
		}
	}

	fn get_elem_pos(&self, event: MouseEvent) -> (i32, i32) {

		// get elements
		let document = window().unwrap().document().unwrap();
		let wrapper = document.get_element_by_id(USER_TREE_WRAPPER_ID).unwrap();
		let drag = document.get_element_by_id(USER_TREE_ID).unwrap();

		// calculate element position based on cursor position and where the element was grabbed
		let mut elem_pos = (self.elem_x + event.client_x() - self.grab_x, self.elem_y + event.client_y() - self.grab_y);

		// element bounds based on wrapper
		let left_bound = wrapper.client_width() - drag.client_width();
		let top_bound = wrapper.client_height() - drag.client_height();
		if elem_pos.0 > 0 { elem_pos.0 = 0 }
		if elem_pos.1 > 0 { elem_pos.1 = 0 }
		if elem_pos.0 < left_bound { elem_pos.0 = left_bound }
		if elem_pos.1 < top_bound { elem_pos.1 = top_bound }

		elem_pos
	}
}