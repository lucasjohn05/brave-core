#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = Gamepad , typescript_type = "Gamepad")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Gamepad` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Gamepad`*"]
    pub type Gamepad;
    # [wasm_bindgen (structural , method , getter , js_class = "Gamepad" , js_name = id)]
    #[doc = "Getter for the `id` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/id)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Gamepad`*"]
    pub fn id(this: &Gamepad) -> ::alloc::string::String;
    # [wasm_bindgen (structural , method , getter , js_class = "Gamepad" , js_name = index)]
    #[doc = "Getter for the `index` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/index)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Gamepad`*"]
    pub fn index(this: &Gamepad) -> u32;
    # [wasm_bindgen (structural , method , getter , js_class = "Gamepad" , js_name = connected)]
    #[doc = "Getter for the `connected` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/connected)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Gamepad`*"]
    pub fn connected(this: &Gamepad) -> bool;
    # [wasm_bindgen (structural , method , getter , js_class = "Gamepad" , js_name = timestamp)]
    #[doc = "Getter for the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/timestamp)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Gamepad`*"]
    pub fn timestamp(this: &Gamepad) -> f64;
    #[cfg(feature = "GamepadMappingType")]
    # [wasm_bindgen (structural , method , getter , js_class = "Gamepad" , js_name = mapping)]
    #[doc = "Getter for the `mapping` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/mapping)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Gamepad`, `GamepadMappingType`*"]
    pub fn mapping(this: &Gamepad) -> GamepadMappingType;
    # [wasm_bindgen (structural , method , getter , js_class = "Gamepad" , js_name = axes)]
    #[doc = "Getter for the `axes` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/axes)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Gamepad`*"]
    pub fn axes(this: &Gamepad) -> ::js_sys::Array;
    # [wasm_bindgen (structural , method , getter , js_class = "Gamepad" , js_name = buttons)]
    #[doc = "Getter for the `buttons` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/buttons)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Gamepad`*"]
    pub fn buttons(this: &Gamepad) -> ::js_sys::Array;
    # [wasm_bindgen (structural , method , getter , js_class = "Gamepad" , js_name = displayId)]
    #[doc = "Getter for the `displayId` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/displayId)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Gamepad`*"]
    #[deprecated]
    pub fn display_id(this: &Gamepad) -> u32;
    #[cfg(feature = "GamepadHand")]
    # [wasm_bindgen (structural , method , getter , js_class = "Gamepad" , js_name = hand)]
    #[doc = "Getter for the `hand` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/hand)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Gamepad`, `GamepadHand`*"]
    pub fn hand(this: &Gamepad) -> GamepadHand;
    # [wasm_bindgen (structural , method , getter , js_class = "Gamepad" , js_name = hapticActuators)]
    #[doc = "Getter for the `hapticActuators` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/hapticActuators)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Gamepad`*"]
    pub fn haptic_actuators(this: &Gamepad) -> ::js_sys::Array;
    #[cfg(feature = "GamepadPose")]
    # [wasm_bindgen (structural , method , getter , js_class = "Gamepad" , js_name = pose)]
    #[doc = "Getter for the `pose` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/pose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Gamepad`, `GamepadPose`*"]
    pub fn pose(this: &Gamepad) -> Option<GamepadPose>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GamepadHapticActuator")]
    # [wasm_bindgen (structural , method , getter , js_class = "Gamepad" , js_name = vibrationActuator)]
    #[doc = "Getter for the `vibrationActuator` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/vibrationActuator)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Gamepad`, `GamepadHapticActuator`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn vibration_actuator(this: &Gamepad) -> GamepadHapticActuator;
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (structural , method , getter , js_class = "Gamepad" , js_name = touchEvents)]
    #[doc = "Getter for the `touchEvents` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/touchEvents)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Gamepad`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn touch_events(this: &Gamepad) -> Option<::js_sys::Array>;
}
