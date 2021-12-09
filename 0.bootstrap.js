(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[0],{

/***/ "../pkg/agents.js":
/*!************************!*\
  !*** ../pkg/agents.js ***!
  \************************/
/*! exports provided: board_width, board_height, run, tick, Cell, __wbindgen_throw */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var _agents_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./agents_bg.wasm */ \"../pkg/agents_bg.wasm\");\n/* harmony import */ var _agents_bg_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./agents_bg.js */ \"../pkg/agents_bg.js\");\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"board_width\", function() { return _agents_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"board_width\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"board_height\", function() { return _agents_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"board_height\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"run\", function() { return _agents_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"run\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"tick\", function() { return _agents_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"tick\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"Cell\", function() { return _agents_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"Cell\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_throw\", function() { return _agents_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_throw\"]; });\n\n\n\n\n//# sourceURL=webpack:///../pkg/agents.js?");

/***/ }),

/***/ "../pkg/agents_bg.js":
/*!***************************!*\
  !*** ../pkg/agents_bg.js ***!
  \***************************/
/*! exports provided: board_width, board_height, run, tick, Cell, __wbindgen_throw */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* WEBPACK VAR INJECTION */(function(module) {/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"board_width\", function() { return board_width; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"board_height\", function() { return board_height; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"run\", function() { return run; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"tick\", function() { return tick; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"Cell\", function() { return Cell; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_throw\", function() { return __wbindgen_throw; });\n/* harmony import */ var _agents_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./agents_bg.wasm */ \"../pkg/agents_bg.wasm\");\n\n\nconst lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;\n\nlet cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });\n\ncachedTextDecoder.decode();\n\nlet cachegetUint8Memory0 = null;\nfunction getUint8Memory0() {\n    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== _agents_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetUint8Memory0 = new Uint8Array(_agents_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetUint8Memory0;\n}\n\nfunction getStringFromWasm0(ptr, len) {\n    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));\n}\n/**\n* @returns {number}\n*/\nfunction board_width() {\n    var ret = _agents_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"board_width\"]();\n    return ret >>> 0;\n}\n\n/**\n* @returns {number}\n*/\nfunction board_height() {\n    var ret = _agents_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"board_height\"]();\n    return ret >>> 0;\n}\n\nlet WASM_VECTOR_LEN = 0;\n\nconst lTextEncoder = typeof TextEncoder === 'undefined' ? (0, module.require)('util').TextEncoder : TextEncoder;\n\nlet cachedTextEncoder = new lTextEncoder('utf-8');\n\nconst encodeString = (typeof cachedTextEncoder.encodeInto === 'function'\n    ? function (arg, view) {\n    return cachedTextEncoder.encodeInto(arg, view);\n}\n    : function (arg, view) {\n    const buf = cachedTextEncoder.encode(arg);\n    view.set(buf);\n    return {\n        read: arg.length,\n        written: buf.length\n    };\n});\n\nfunction passStringToWasm0(arg, malloc, realloc) {\n\n    if (realloc === undefined) {\n        const buf = cachedTextEncoder.encode(arg);\n        const ptr = malloc(buf.length);\n        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);\n        WASM_VECTOR_LEN = buf.length;\n        return ptr;\n    }\n\n    let len = arg.length;\n    let ptr = malloc(len);\n\n    const mem = getUint8Memory0();\n\n    let offset = 0;\n\n    for (; offset < len; offset++) {\n        const code = arg.charCodeAt(offset);\n        if (code > 0x7F) break;\n        mem[ptr + offset] = code;\n    }\n\n    if (offset !== len) {\n        if (offset !== 0) {\n            arg = arg.slice(offset);\n        }\n        ptr = realloc(ptr, len, len = offset + arg.length * 3);\n        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);\n        const ret = encodeString(arg, view);\n\n        offset += ret.written;\n    }\n\n    WASM_VECTOR_LEN = offset;\n    return ptr;\n}\n/**\n* @param {string} source\n*/\nfunction run(source) {\n    var ptr0 = passStringToWasm0(source, _agents_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"], _agents_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_realloc\"]);\n    var len0 = WASM_VECTOR_LEN;\n    _agents_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"run\"](ptr0, len0);\n}\n\nlet cachegetInt32Memory0 = null;\nfunction getInt32Memory0() {\n    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== _agents_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetInt32Memory0 = new Int32Array(_agents_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetInt32Memory0;\n}\n\nlet cachegetUint32Memory0 = null;\nfunction getUint32Memory0() {\n    if (cachegetUint32Memory0 === null || cachegetUint32Memory0.buffer !== _agents_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetUint32Memory0 = new Uint32Array(_agents_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetUint32Memory0;\n}\n\nfunction getArrayU32FromWasm0(ptr, len) {\n    return getUint32Memory0().subarray(ptr / 4, ptr / 4 + len);\n}\n/**\n* @returns {Uint32Array}\n*/\nfunction tick() {\n    try {\n        const retptr = _agents_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_add_to_stack_pointer\"](-16);\n        _agents_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"tick\"](retptr);\n        var r0 = getInt32Memory0()[retptr / 4 + 0];\n        var r1 = getInt32Memory0()[retptr / 4 + 1];\n        var v0 = getArrayU32FromWasm0(r0, r1).slice();\n        _agents_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_free\"](r0, r1 * 4);\n        return v0;\n    } finally {\n        _agents_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_add_to_stack_pointer\"](16);\n    }\n}\n\n/**\n*/\nclass Cell {\n\n    __destroy_into_raw() {\n        const ptr = this.ptr;\n        this.ptr = 0;\n\n        return ptr;\n    }\n\n    free() {\n        const ptr = this.__destroy_into_raw();\n        _agents_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbg_cell_free\"](ptr);\n    }\n}\n\nfunction __wbindgen_throw(arg0, arg1) {\n    throw new Error(getStringFromWasm0(arg0, arg1));\n};\n\n\n/* WEBPACK VAR INJECTION */}.call(this, __webpack_require__(/*! ./../www/node_modules/webpack/buildin/harmony-module.js */ \"./node_modules/webpack/buildin/harmony-module.js\")(module)))\n\n//# sourceURL=webpack:///../pkg/agents_bg.js?");

/***/ }),

/***/ "../pkg/agents_bg.wasm":
/*!*****************************!*\
  !*** ../pkg/agents_bg.wasm ***!
  \*****************************/
/*! exports provided: memory, __wbg_cell_free, board_width, board_height, run, tick, __wbindgen_malloc, __wbindgen_realloc, __wbindgen_add_to_stack_pointer, __wbindgen_free */
/***/ (function(module, exports, __webpack_require__) {

eval("\"use strict\";\n// Instantiate WebAssembly module\nvar wasmExports = __webpack_require__.w[module.i];\n__webpack_require__.r(exports);\n// export exports from WebAssembly module\nfor(var name in wasmExports) if(name != \"__webpack_init__\") exports[name] = wasmExports[name];\n// exec imports from WebAssembly module (for esm order)\n/* harmony import */ var m0 = __webpack_require__(/*! ./agents_bg.js */ \"../pkg/agents_bg.js\");\n\n\n// exec wasm module\nwasmExports[\"__webpack_init__\"]()\n\n//# sourceURL=webpack:///../pkg/agents_bg.wasm?");

/***/ }),

/***/ "./index.js":
/*!******************!*\
  !*** ./index.js ***!
  \******************/
/*! no exports provided */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var agents__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! agents */ \"../pkg/agents.js\");\n\n\nconst CELL_SIZE = 5;\nconst GRID_COLOR = \"#CCCCCC\";\n\n// Construct the universe, and get its width and height.\nconst width = agents__WEBPACK_IMPORTED_MODULE_0__[\"board_width\"]();\nconst height = agents__WEBPACK_IMPORTED_MODULE_0__[\"board_height\"]();\n\nconst RED_MASK = 0xff << 16;\nconst GREEN_MASK = 0xff << 8;\nconst BLUE_MASK = 0xff;\n\n// Give the canvas room for all of our cells and a 1px border\n// around each of them.\nconst canvas = document.getElementById('canvas');\ncanvas.height = (CELL_SIZE + 1) * height + 1;\ncanvas.width = (CELL_SIZE + 1) * width + 1;\n\nconst ctx = canvas.getContext('2d');\n\nfunction drawGrid() {\n  ctx.beginPath();\n  ctx.strokeStyle = GRID_COLOR;\n\n  // Vertical lines.\n  for (let i = 0; i <= width; i++) {\n    let x = i * (CELL_SIZE + 1) + 1;\n    let y = (CELL_SIZE + 1) * height + 1;\n    ctx.moveTo(x, 0);\n    ctx.lineTo(x, y);\n  }\n\n  // Horizontal lines.\n  for (let j = 0; j <= height; j++) {\n    let x = (CELL_SIZE + 1) * width + 1;\n    let y = j * (CELL_SIZE + 1) + 1;\n    ctx.moveTo(0, y);\n    ctx.lineTo(x, y);\n  }\n\n  ctx.stroke();\n\n  let colors = agents__WEBPACK_IMPORTED_MODULE_0__[\"tick\"]();\n  for (let y = 0; y < height; y++) {\n    for (let x = 0; x < width; x++) {\n      let i = y * height + x;\n      let color = colors[i];\n      ctx.fillStyle = 'rgb(' + (color & RED_MASK).toString() + ',' + (color & GREEN_MASK).toString() + ',' + (color & BLUE_MASK).toString() + ')';\n      ctx.fillRect((CELL_SIZE + 1) * x + 2, (CELL_SIZE + 1) * y + 2, CELL_SIZE - 1, CELL_SIZE - 1);\n    }\n  }\n}\n\nlet fps = 10;\n\nfunction loop() {\n  drawGrid();\n\n  setTimeout(() => {\n    requestAnimationFrame(loop);\n  }, 1000.0 / fps);\n}\n\ndocument.getElementById('runcode').addEventListener('click', () => {\n  agents__WEBPACK_IMPORTED_MODULE_0__[\"run\"](editor.getValue());\n});\n\nloop();\n\ndocument.getElementById('editor').innerHTML = `# A program is composed of one or more independent agents.\n# Agents move around the board in the direction they are oriented in on cycle\n# of ticks, leaving behind a colored wake in their path.\n#\n# Agents are defined by by using the \"agent\" keyword followed by a name and\n# semicolon.\nagent red_agent:\n    # Everything in a agent block is a statement.\n    #\n    # Statements can be labels or commands.\n    # \n    # One such command is \"set\", used for defining variables.\n    # Some variables are special and impact an agents placement or effect on\n    # the board.\n    # These include color, x and y for cell color or coordinates.\n    set color = 0xFF0000\n    set x = 40\n    set y = 40\n    # Other variables can be general purpose.\n    set acc = 1\n    # Commands control either agent behavior or control flow.\n    #\n    # face changes orientation of an agent and takes a cardinal direction, like\n    # S, E or NW.\n    face NW\n    # Labels allow defining locations in code for a program to jump to\n    loop:\n        # Move takes a positive 32 bit integer and controls the number of steps\n        # to move in a tick in this case our agent will move one space per tick\n        # in the direction they are facing.\n        move 1\n        # Agent scripts support branching through the \"jump to\" command.\n        # These commands look like:\n        # \"jump to <label> if <expression> is <expresion>\".\n        jump to spin if acc is 4\n        # Variables also support basic arithmetic expressions.\n        set acc = acc + 1\n        # The goto command allows for the basic jumping to a label without a\n        # condition.\n        goto loop\n    spin:\n        # The turn command allows spinning an agent based on their current \n        # orientation. With a positive number being a clockwise rotation and a\n        # negative being counter-clockwise.\n        turn 1\n        set acc = 1\n        goto loop\nagent blue_agent:\n    set color = 0xFF\n    set x = 40\n    set y = 40\n    face NE\n    loop:\n        move 2\n        goto loop\n`\n\nlet editor = ace.edit('editor');\neditor.setTheme(\"ace/theme/monokai\");\n\n\n//# sourceURL=webpack:///./index.js?");

/***/ }),

/***/ "./node_modules/webpack/buildin/harmony-module.js":
/*!*******************************************!*\
  !*** (webpack)/buildin/harmony-module.js ***!
  \*******************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("module.exports = function(originalModule) {\n\tif (!originalModule.webpackPolyfill) {\n\t\tvar module = Object.create(originalModule);\n\t\t// module.parent = undefined by default\n\t\tif (!module.children) module.children = [];\n\t\tObject.defineProperty(module, \"loaded\", {\n\t\t\tenumerable: true,\n\t\t\tget: function() {\n\t\t\t\treturn module.l;\n\t\t\t}\n\t\t});\n\t\tObject.defineProperty(module, \"id\", {\n\t\t\tenumerable: true,\n\t\t\tget: function() {\n\t\t\t\treturn module.i;\n\t\t\t}\n\t\t});\n\t\tObject.defineProperty(module, \"exports\", {\n\t\t\tenumerable: true\n\t\t});\n\t\tmodule.webpackPolyfill = 1;\n\t}\n\treturn module;\n};\n\n\n//# sourceURL=webpack:///(webpack)/buildin/harmony-module.js?");

/***/ })

}]);