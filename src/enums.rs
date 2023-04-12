//! This module contains auto-generated Rust representations of raylib's enums.

/// System/Window config flags
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ConfigFlags {
	/// Set to try enabling V-Sync on GPU
	FlagVsyncHint = 64,
	/// Set to run program in fullscreen
	FlagFullscreenMode = 2,
	/// Set to allow resizable window
	FlagWindowResizable = 4,
	/// Set to disable window decoration (frame and buttons)
	FlagWindowUndecorated = 8,
	/// Set to hide window
	FlagWindowHidden = 128,
	/// Set to minimize window (iconify)
	FlagWindowMinimized = 512,
	/// Set to maximize window (expanded to monitor)
	FlagWindowMaximized = 1024,
	/// Set to window non focused
	FlagWindowUnfocused = 2048,
	/// Set to window always on top
	FlagWindowTopmost = 4096,
	/// Set to allow windows running while minimized
	FlagWindowAlwaysRun = 256,
	/// Set to allow transparent framebuffer
	FlagWindowTransparent = 16,
	/// Set to support HighDPI
	FlagWindowHighdpi = 8192,
	/// Set to support mouse passthrough, only supported when FLAG_WINDOW_UNDECORATED
	FlagWindowMousePassthrough = 16384,
	/// Set to try enabling MSAA 4X
	FlagMsaa4xHint = 32,
	/// Set to try enabling interlaced video format (for V3D)
	FlagInterlacedHint = 65536,
}

/// Trace log level
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TraceLogLevel {
	/// Display all logs
	LogAll = 0,
	/// Trace logging, intended for internal use only
	LogTrace = 1,
	/// Debug logging, used for internal debugging, it should be disabled on release builds
	LogDebug = 2,
	/// Info logging, used for program execution info
	LogInfo = 3,
	/// Warning logging, used on recoverable failures
	LogWarning = 4,
	/// Error logging, used on unrecoverable failures
	LogError = 5,
	/// Fatal logging, used to abort program: exit(EXIT_FAILURE)
	LogFatal = 6,
	/// Disable logging
	LogNone = 7,
}

/// Keyboard keys (US keyboard layout)
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum KeyboardKey {
	/// Key: NULL, used for no key pressed
	KeyNull = 0,
	/// Key: '
	KeyApostrophe = 39,
	/// Key: ,
	KeyComma = 44,
	/// Key: -
	KeyMinus = 45,
	/// Key: .
	KeyPeriod = 46,
	/// Key: /
	KeySlash = 47,
	/// Key: 0
	KeyZero = 48,
	/// Key: 1
	KeyOne = 49,
	/// Key: 2
	KeyTwo = 50,
	/// Key: 3
	KeyThree = 51,
	/// Key: 4
	KeyFour = 52,
	/// Key: 5
	KeyFive = 53,
	/// Key: 6
	KeySix = 54,
	/// Key: 7
	KeySeven = 55,
	/// Key: 8
	KeyEight = 56,
	/// Key: 9
	KeyNine = 57,
	/// Key: ;
	KeySemicolon = 59,
	/// Key: =
	KeyEqual = 61,
	/// Key: A | a
	KeyA = 65,
	/// Key: B | b
	KeyB = 66,
	/// Key: C | c
	KeyC = 67,
	/// Key: D | d
	KeyD = 68,
	/// Key: E | e
	KeyE = 69,
	/// Key: F | f
	KeyF = 70,
	/// Key: G | g
	KeyG = 71,
	/// Key: H | h
	KeyH = 72,
	/// Key: I | i
	KeyI = 73,
	/// Key: J | j
	KeyJ = 74,
	/// Key: K | k
	KeyK = 75,
	/// Key: L | l
	KeyL = 76,
	/// Key: M | m
	KeyM = 77,
	/// Key: N | n
	KeyN = 78,
	/// Key: O | o
	KeyO = 79,
	/// Key: P | p
	KeyP = 80,
	/// Key: Q | q
	KeyQ = 81,
	/// Key: R | r
	KeyR = 82,
	/// Key: S | s
	KeyS = 83,
	/// Key: T | t
	KeyT = 84,
	/// Key: U | u
	KeyU = 85,
	/// Key: V | v
	KeyV = 86,
	/// Key: W | w
	KeyW = 87,
	/// Key: X | x
	KeyX = 88,
	/// Key: Y | y
	KeyY = 89,
	/// Key: Z | z
	KeyZ = 90,
	/// Key: [
	KeyLeftBracket = 91,
	/// Key: '\'
	KeyBackslash = 92,
	/// Key: ]
	KeyRightBracket = 93,
	/// Key: `
	KeyGrave = 96,
	/// Key: Space
	KeySpace = 32,
	/// Key: Esc
	KeyEscape = 256,
	/// Key: Enter
	KeyEnter = 257,
	/// Key: Tab
	KeyTab = 258,
	/// Key: Backspace
	KeyBackspace = 259,
	/// Key: Ins
	KeyInsert = 260,
	/// Key: Del
	KeyDelete = 261,
	/// Key: Cursor right
	KeyRight = 262,
	/// Key: Cursor left
	KeyLeft = 263,
	/// Key: Cursor down
	KeyDown = 264,
	/// Key: Cursor up
	KeyUp = 265,
	/// Key: Page up
	KeyPageUp = 266,
	/// Key: Page down
	KeyPageDown = 267,
	/// Key: Home
	KeyHome = 268,
	/// Key: End
	KeyEnd = 269,
	/// Key: Caps lock
	KeyCapsLock = 280,
	/// Key: Scroll down
	KeyScrollLock = 281,
	/// Key: Num lock
	KeyNumLock = 282,
	/// Key: Print screen
	KeyPrintScreen = 283,
	/// Key: Pause
	KeyPause = 284,
	/// Key: F1
	KeyF1 = 290,
	/// Key: F2
	KeyF2 = 291,
	/// Key: F3
	KeyF3 = 292,
	/// Key: F4
	KeyF4 = 293,
	/// Key: F5
	KeyF5 = 294,
	/// Key: F6
	KeyF6 = 295,
	/// Key: F7
	KeyF7 = 296,
	/// Key: F8
	KeyF8 = 297,
	/// Key: F9
	KeyF9 = 298,
	/// Key: F10
	KeyF10 = 299,
	/// Key: F11
	KeyF11 = 300,
	/// Key: F12
	KeyF12 = 301,
	/// Key: Shift left
	KeyLeftShift = 340,
	/// Key: Control left
	KeyLeftControl = 341,
	/// Key: Alt left
	KeyLeftAlt = 342,
	/// Key: Super left
	KeyLeftSuper = 343,
	/// Key: Shift right
	KeyRightShift = 344,
	/// Key: Control right
	KeyRightControl = 345,
	/// Key: Alt right
	KeyRightAlt = 346,
	/// Key: Super right
	KeyRightSuper = 347,
	/// Key: KB menu
	KeyKbMenu = 348,
	/// Key: Keypad 0
	KeyKp0 = 320,
	/// Key: Keypad 1
	KeyKp1 = 321,
	/// Key: Keypad 2
	KeyKp2 = 322,
	/// Key: Keypad 3
	KeyKp3 = 323,
	/// Key: Keypad 4
	KeyKp4 = 324,
	/// Key: Keypad 5
	KeyKp5 = 325,
	/// Key: Keypad 6
	KeyKp6 = 326,
	/// Key: Keypad 7
	KeyKp7 = 327,
	/// Key: Keypad 8
	KeyKp8 = 328,
	/// Key: Keypad 9
	KeyKp9 = 329,
	/// Key: Keypad .
	KeyKpDecimal = 330,
	/// Key: Keypad /
	KeyKpDivide = 331,
	/// Key: Keypad *
	KeyKpMultiply = 332,
	/// Key: Keypad -
	KeyKpSubtract = 333,
	/// Key: Keypad +
	KeyKpAdd = 334,
	/// Key: Keypad Enter
	KeyKpEnter = 335,
	/// Key: Keypad =
	KeyKpEqual = 336,
	/// Key: Android back button
	KeyBack = 4,
	/// Key: Android volume up button
	KeyVolumeUp = 24,
	/// Key: Android volume down button
	KeyVolumeDown = 25,
}

/// Mouse buttons
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MouseButton {
	/// Mouse button left
	MouseButtonLeft = 0,
	/// Mouse button right
	MouseButtonRight = 1,
	/// Mouse button middle (pressed wheel)
	MouseButtonMiddle = 2,
	/// Mouse button side (advanced mouse device)
	MouseButtonSide = 3,
	/// Mouse button extra (advanced mouse device)
	MouseButtonExtra = 4,
	/// Mouse button forward (advanced mouse device)
	MouseButtonForward = 5,
	/// Mouse button back (advanced mouse device)
	MouseButtonBack = 6,
}

/// Mouse cursor
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MouseCursor {
	/// Default pointer shape
	MouseCursorDefault = 0,
	/// Arrow shape
	MouseCursorArrow = 1,
	/// Text writing cursor shape
	MouseCursorIbeam = 2,
	/// Cross shape
	MouseCursorCrosshair = 3,
	/// Pointing hand cursor
	MouseCursorPointingHand = 4,
	/// Horizontal resize/move arrow shape
	MouseCursorResizeEw = 5,
	/// Vertical resize/move arrow shape
	MouseCursorResizeNs = 6,
	/// Top-left to bottom-right diagonal resize/move arrow shape
	MouseCursorResizeNwse = 7,
	/// The top-right to bottom-left diagonal resize/move arrow shape
	MouseCursorResizeNesw = 8,
	/// The omnidirectional resize/move cursor shape
	MouseCursorResizeAll = 9,
	/// The operation-not-allowed shape
	MouseCursorNotAllowed = 10,
}

/// Gamepad buttons
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum GamepadButton {
	/// Unknown button, just for error checking
	GamepadButtonUnknown = 0,
	/// Gamepad left DPAD up button
	GamepadButtonLeftFaceUp = 1,
	/// Gamepad left DPAD right button
	GamepadButtonLeftFaceRight = 2,
	/// Gamepad left DPAD down button
	GamepadButtonLeftFaceDown = 3,
	/// Gamepad left DPAD left button
	GamepadButtonLeftFaceLeft = 4,
	/// Gamepad right button up (i.e. PS3: Triangle, Xbox: Y)
	GamepadButtonRightFaceUp = 5,
	/// Gamepad right button right (i.e. PS3: Square, Xbox: X)
	GamepadButtonRightFaceRight = 6,
	/// Gamepad right button down (i.e. PS3: Cross, Xbox: A)
	GamepadButtonRightFaceDown = 7,
	/// Gamepad right button left (i.e. PS3: Circle, Xbox: B)
	GamepadButtonRightFaceLeft = 8,
	/// Gamepad top/back trigger left (first), it could be a trailing button
	GamepadButtonLeftTrigger1 = 9,
	/// Gamepad top/back trigger left (second), it could be a trailing button
	GamepadButtonLeftTrigger2 = 10,
	/// Gamepad top/back trigger right (one), it could be a trailing button
	GamepadButtonRightTrigger1 = 11,
	/// Gamepad top/back trigger right (second), it could be a trailing button
	GamepadButtonRightTrigger2 = 12,
	/// Gamepad center buttons, left one (i.e. PS3: Select)
	GamepadButtonMiddleLeft = 13,
	/// Gamepad center buttons, middle one (i.e. PS3: PS, Xbox: XBOX)
	GamepadButtonMiddle = 14,
	/// Gamepad center buttons, right one (i.e. PS3: Start)
	GamepadButtonMiddleRight = 15,
	/// Gamepad joystick pressed button left
	GamepadButtonLeftThumb = 16,
	/// Gamepad joystick pressed button right
	GamepadButtonRightThumb = 17,
}

/// Gamepad axis
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum GamepadAxis {
	/// Gamepad left stick X axis
	GamepadAxisLeftX = 0,
	/// Gamepad left stick Y axis
	GamepadAxisLeftY = 1,
	/// Gamepad right stick X axis
	GamepadAxisRightX = 2,
	/// Gamepad right stick Y axis
	GamepadAxisRightY = 3,
	/// Gamepad back trigger left, pressure level: [1..-1]
	GamepadAxisLeftTrigger = 4,
	/// Gamepad back trigger right, pressure level: [1..-1]
	GamepadAxisRightTrigger = 5,
}

/// Material map index
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MaterialMapIndex {
	/// Albedo material (same as: MATERIAL_MAP_DIFFUSE)
	MaterialMapAlbedo = 0,
	/// Metalness material (same as: MATERIAL_MAP_SPECULAR)
	MaterialMapMetalness = 1,
	/// Normal material
	MaterialMapNormal = 2,
	/// Roughness material
	MaterialMapRoughness = 3,
	/// Ambient occlusion material
	MaterialMapOcclusion = 4,
	/// Emission material
	MaterialMapEmission = 5,
	/// Heightmap material
	MaterialMapHeight = 6,
	/// Cubemap material (NOTE: Uses GL_TEXTURE_CUBE_MAP)
	MaterialMapCubemap = 7,
	/// Irradiance material (NOTE: Uses GL_TEXTURE_CUBE_MAP)
	MaterialMapIrradiance = 8,
	/// Prefilter material (NOTE: Uses GL_TEXTURE_CUBE_MAP)
	MaterialMapPrefilter = 9,
	/// Brdf material
	MaterialMapBrdf = 10,
}

/// Shader location index
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ShaderLocationIndex {
	/// Shader location: vertex attribute: position
	ShaderLocVertexPosition = 0,
	/// Shader location: vertex attribute: texcoord01
	ShaderLocVertexTexcoord01 = 1,
	/// Shader location: vertex attribute: texcoord02
	ShaderLocVertexTexcoord02 = 2,
	/// Shader location: vertex attribute: normal
	ShaderLocVertexNormal = 3,
	/// Shader location: vertex attribute: tangent
	ShaderLocVertexTangent = 4,
	/// Shader location: vertex attribute: color
	ShaderLocVertexColor = 5,
	/// Shader location: matrix uniform: model-view-projection
	ShaderLocMatrixMvp = 6,
	/// Shader location: matrix uniform: view (camera transform)
	ShaderLocMatrixView = 7,
	/// Shader location: matrix uniform: projection
	ShaderLocMatrixProjection = 8,
	/// Shader location: matrix uniform: model (transform)
	ShaderLocMatrixModel = 9,
	/// Shader location: matrix uniform: normal
	ShaderLocMatrixNormal = 10,
	/// Shader location: vector uniform: view
	ShaderLocVectorView = 11,
	/// Shader location: vector uniform: diffuse color
	ShaderLocColorDiffuse = 12,
	/// Shader location: vector uniform: specular color
	ShaderLocColorSpecular = 13,
	/// Shader location: vector uniform: ambient color
	ShaderLocColorAmbient = 14,
	/// Shader location: sampler2d texture: albedo (same as: SHADER_LOC_MAP_DIFFUSE)
	ShaderLocMapAlbedo = 15,
	/// Shader location: sampler2d texture: metalness (same as: SHADER_LOC_MAP_SPECULAR)
	ShaderLocMapMetalness = 16,
	/// Shader location: sampler2d texture: normal
	ShaderLocMapNormal = 17,
	/// Shader location: sampler2d texture: roughness
	ShaderLocMapRoughness = 18,
	/// Shader location: sampler2d texture: occlusion
	ShaderLocMapOcclusion = 19,
	/// Shader location: sampler2d texture: emission
	ShaderLocMapEmission = 20,
	/// Shader location: sampler2d texture: height
	ShaderLocMapHeight = 21,
	/// Shader location: samplerCube texture: cubemap
	ShaderLocMapCubemap = 22,
	/// Shader location: samplerCube texture: irradiance
	ShaderLocMapIrradiance = 23,
	/// Shader location: samplerCube texture: prefilter
	ShaderLocMapPrefilter = 24,
	/// Shader location: sampler2d texture: brdf
	ShaderLocMapBrdf = 25,
}

/// Shader uniform data type
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ShaderUniformDataType {
	/// Shader uniform type: float
	ShaderUniformFloat = 0,
	/// Shader uniform type: vec2 (2 float)
	ShaderUniformVec2 = 1,
	/// Shader uniform type: vec3 (3 float)
	ShaderUniformVec3 = 2,
	/// Shader uniform type: vec4 (4 float)
	ShaderUniformVec4 = 3,
	/// Shader uniform type: int
	ShaderUniformInt = 4,
	/// Shader uniform type: ivec2 (2 int)
	ShaderUniformIvec2 = 5,
	/// Shader uniform type: ivec3 (3 int)
	ShaderUniformIvec3 = 6,
	/// Shader uniform type: ivec4 (4 int)
	ShaderUniformIvec4 = 7,
	/// Shader uniform type: sampler2d
	ShaderUniformSampler2d = 8,
}

/// Shader attribute data types
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ShaderAttributeDataType {
	/// Shader attribute type: float
	ShaderAttribFloat = 0,
	/// Shader attribute type: vec2 (2 float)
	ShaderAttribVec2 = 1,
	/// Shader attribute type: vec3 (3 float)
	ShaderAttribVec3 = 2,
	/// Shader attribute type: vec4 (4 float)
	ShaderAttribVec4 = 3,
}

/// Pixel formats
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PixelFormat {
	/// 8 bit per pixel (no alpha)
	PixelformatUncompressedGrayscale = 1,
	/// 8*2 bpp (2 channels)
	PixelformatUncompressedGrayAlpha = 2,
	/// 16 bpp
	PixelformatUncompressedR5g6b5 = 3,
	/// 24 bpp
	PixelformatUncompressedR8g8b8 = 4,
	/// 16 bpp (1 bit alpha)
	PixelformatUncompressedR5g5b5a1 = 5,
	/// 16 bpp (4 bit alpha)
	PixelformatUncompressedR4g4b4a4 = 6,
	/// 32 bpp
	PixelformatUncompressedR8g8b8a8 = 7,
	/// 32 bpp (1 channel - float)
	PixelformatUncompressedR32 = 8,
	/// 32*3 bpp (3 channels - float)
	PixelformatUncompressedR32g32b32 = 9,
	/// 32*4 bpp (4 channels - float)
	PixelformatUncompressedR32g32b32a32 = 10,
	/// 4 bpp (no alpha)
	PixelformatCompressedDxt1Rgb = 11,
	/// 4 bpp (1 bit alpha)
	PixelformatCompressedDxt1Rgba = 12,
	/// 8 bpp
	PixelformatCompressedDxt3Rgba = 13,
	/// 8 bpp
	PixelformatCompressedDxt5Rgba = 14,
	/// 4 bpp
	PixelformatCompressedEtc1Rgb = 15,
	/// 4 bpp
	PixelformatCompressedEtc2Rgb = 16,
	/// 8 bpp
	PixelformatCompressedEtc2EacRgba = 17,
	/// 4 bpp
	PixelformatCompressedPvrtRgb = 18,
	/// 4 bpp
	PixelformatCompressedPvrtRgba = 19,
	/// 8 bpp
	PixelformatCompressedAstc4x4Rgba = 20,
	/// 2 bpp
	PixelformatCompressedAstc8x8Rgba = 21,
}

/// Texture parameters: filter mode
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TextureFilter {
	/// No filter, just pixel approximation
	TextureFilterPoint = 0,
	/// Linear filtering
	TextureFilterBilinear = 1,
	/// Trilinear filtering (linear with mipmaps)
	TextureFilterTrilinear = 2,
	/// Anisotropic filtering 4x
	TextureFilterAnisotropic4x = 3,
	/// Anisotropic filtering 8x
	TextureFilterAnisotropic8x = 4,
	/// Anisotropic filtering 16x
	TextureFilterAnisotropic16x = 5,
}

/// Texture parameters: wrap mode
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TextureWrap {
	/// Repeats texture in tiled mode
	TextureWrapRepeat = 0,
	/// Clamps texture to edge pixel in tiled mode
	TextureWrapClamp = 1,
	/// Mirrors and repeats the texture in tiled mode
	TextureWrapMirrorRepeat = 2,
	/// Mirrors and clamps to border the texture in tiled mode
	TextureWrapMirrorClamp = 3,
}

/// Cubemap layouts
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CubemapLayout {
	/// Automatically detect layout type
	CubemapLayoutAutoDetect = 0,
	/// Layout is defined by a vertical line with faces
	CubemapLayoutLineVertical = 1,
	/// Layout is defined by a horizontal line with faces
	CubemapLayoutLineHorizontal = 2,
	/// Layout is defined by a 3x4 cross with cubemap faces
	CubemapLayoutCrossThreeByFour = 3,
	/// Layout is defined by a 4x3 cross with cubemap faces
	CubemapLayoutCrossFourByThree = 4,
	/// Layout is defined by a panorama image (equirrectangular map)
	CubemapLayoutPanorama = 5,
}

/// Font type, defines generation method
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum FontType {
	/// Default font generation, anti-aliased
	FontDefault = 0,
	/// Bitmap font generation, no anti-aliasing
	FontBitmap = 1,
	/// SDF font generation, requires external shader
	FontSdf = 2,
}

/// Color blending modes (pre-defined)
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BlendMode {
	/// Blend textures considering alpha (default)
	BlendAlpha = 0,
	/// Blend textures adding colors
	BlendAdditive = 1,
	/// Blend textures multiplying colors
	BlendMultiplied = 2,
	/// Blend textures adding colors (alternative)
	BlendAddColors = 3,
	/// Blend textures subtracting colors (alternative)
	BlendSubtractColors = 4,
	/// Blend premultiplied textures considering alpha
	BlendAlphaPremultiply = 5,
	/// Blend textures using custom src/dst factors (use rlSetBlendFactors())
	BlendCustom = 6,
	/// Blend textures using custom rgb/alpha separate src/dst factors (use rlSetBlendFactorsSeparate())
	BlendCustomSeparate = 7,
}

/// Gesture
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Gesture {
	/// No gesture
	GestureNone = 0,
	/// Tap gesture
	GestureTap = 1,
	/// Double tap gesture
	GestureDoubletap = 2,
	/// Hold gesture
	GestureHold = 4,
	/// Drag gesture
	GestureDrag = 8,
	/// Swipe right gesture
	GestureSwipeRight = 16,
	/// Swipe left gesture
	GestureSwipeLeft = 32,
	/// Swipe up gesture
	GestureSwipeUp = 64,
	/// Swipe down gesture
	GestureSwipeDown = 128,
	/// Pinch in gesture
	GesturePinchIn = 256,
	/// Pinch out gesture
	GesturePinchOut = 512,
}

/// Camera system modes
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CameraMode {
	/// Custom camera
	CameraCustom = 0,
	/// Free camera
	CameraFree = 1,
	/// Orbital camera
	CameraOrbital = 2,
	/// First person camera
	CameraFirstPerson = 3,
	/// Third person camera
	CameraThirdPerson = 4,
}

/// Camera projection
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CameraProjection {
	/// Perspective projection
	CameraPerspective = 0,
	/// Orthographic projection
	CameraOrthographic = 1,
}

/// N-patch layout
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum NPatchLayout {
	/// Npatch layout: 3x3 tiles
	NpatchNinePatch = 0,
	/// Npatch layout: 1x3 tiles
	NpatchThreePatchVertical = 1,
	/// Npatch layout: 3x1 tiles
	NpatchThreePatchHorizontal = 2,
}
