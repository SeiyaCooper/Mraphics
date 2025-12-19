let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_externrefs.set(idx, obj);
    return idx;
}

const CLOSURE_DTORS = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(state => state.dtor(state.a, state.b));

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches && builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

function getArrayU32FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint32ArrayMemory0().subarray(ptr / 4, ptr / 4 + len);
}

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

let cachedDataViewMemory0 = null;
function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return decodeText(ptr, len);
}

let cachedUint32ArrayMemory0 = null;
function getUint32ArrayMemory0() {
    if (cachedUint32ArrayMemory0 === null || cachedUint32ArrayMemory0.byteLength === 0) {
        cachedUint32ArrayMemory0 = new Uint32Array(wasm.memory.buffer);
    }
    return cachedUint32ArrayMemory0;
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        const idx = addToExternrefTable0(e);
        wasm.__wbindgen_exn_store(idx);
    }
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

function makeMutClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1, dtor };
    const real = (...args) => {

        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        const a = state.a;
        state.a = 0;
        try {
            return f(a, state.b, ...args);
        } finally {
            state.a = a;
            real._wbg_cb_unref();
        }
    };
    real._wbg_cb_unref = () => {
        if (--state.cnt === 0) {
            state.dtor(state.a, state.b);
            state.a = 0;
            CLOSURE_DTORS.unregister(state);
        }
    };
    CLOSURE_DTORS.register(real, state, state);
    return real;
}

function passStringToWasm0(arg, malloc, realloc) {
    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }
    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = cachedTextEncoder.encodeInto(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();
const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

const cachedTextEncoder = new TextEncoder();

if (!('encodeInto' in cachedTextEncoder)) {
    cachedTextEncoder.encodeInto = function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    }
}

let WASM_VECTOR_LEN = 0;

function wasm_bindgen__convert__closures_____invoke__hfce13e71b68b9f78(arg0, arg1, arg2) {
    wasm.wasm_bindgen__convert__closures_____invoke__hfce13e71b68b9f78(arg0, arg1, arg2);
}

function wasm_bindgen__convert__closures_____invoke__h5997bd2378143005(arg0, arg1, arg2) {
    wasm.wasm_bindgen__convert__closures_____invoke__h5997bd2378143005(arg0, arg1, arg2);
}

function wasm_bindgen__convert__closures_____invoke__h72e1e188793d6dc6(arg0, arg1, arg2) {
    wasm.wasm_bindgen__convert__closures_____invoke__h72e1e188793d6dc6(arg0, arg1, arg2);
}

function wasm_bindgen__convert__closures_____invoke__hda95989a6850e8ae(arg0, arg1, arg2) {
    wasm.wasm_bindgen__convert__closures_____invoke__hda95989a6850e8ae(arg0, arg1, arg2);
}

function wasm_bindgen__convert__closures_____invoke__h7b85597c6c141fa5(arg0, arg1, arg2, arg3) {
    wasm.wasm_bindgen__convert__closures_____invoke__h7b85597c6c141fa5(arg0, arg1, arg2, arg3);
}

function wasm_bindgen__convert__closures_____invoke__hb5c2a4e092461377(arg0, arg1, arg2) {
    wasm.wasm_bindgen__convert__closures_____invoke__hb5c2a4e092461377(arg0, arg1, arg2);
}

function wasm_bindgen__convert__closures_____invoke__h5bdae1983e88d88e(arg0, arg1, arg2) {
    wasm.wasm_bindgen__convert__closures_____invoke__h5bdae1983e88d88e(arg0, arg1, arg2);
}

function wasm_bindgen__convert__closures_____invoke__hd7f2cee65b407601(arg0, arg1, arg2) {
    wasm.wasm_bindgen__convert__closures_____invoke__hd7f2cee65b407601(arg0, arg1, arg2);
}

function wasm_bindgen__convert__closures_____invoke__h952098ab27c9525c(arg0, arg1, arg2) {
    wasm.wasm_bindgen__convert__closures_____invoke__h952098ab27c9525c(arg0, arg1, arg2);
}

function wasm_bindgen__convert__closures_____invoke__hdb80f1ed7129c4f1(arg0, arg1) {
    wasm.wasm_bindgen__convert__closures_____invoke__hdb80f1ed7129c4f1(arg0, arg1);
}

function wasm_bindgen__convert__closures_____invoke__h9233806efcfd5643(arg0, arg1, arg2) {
    wasm.wasm_bindgen__convert__closures_____invoke__h9233806efcfd5643(arg0, arg1, arg2);
}

function wasm_bindgen__convert__closures_____invoke__hd9dfa106a4aaa8b7(arg0, arg1) {
    const ret = wasm.wasm_bindgen__convert__closures_____invoke__hd9dfa106a4aaa8b7(arg0, arg1);
    return ret !== 0;
}

const __wbindgen_enum_GpuAddressMode = ["clamp-to-edge", "repeat", "mirror-repeat"];

const __wbindgen_enum_GpuBlendFactor = ["zero", "one", "src", "one-minus-src", "src-alpha", "one-minus-src-alpha", "dst", "one-minus-dst", "dst-alpha", "one-minus-dst-alpha", "src-alpha-saturated", "constant", "one-minus-constant", "src1", "one-minus-src1", "src1-alpha", "one-minus-src1-alpha"];

const __wbindgen_enum_GpuBlendOperation = ["add", "subtract", "reverse-subtract", "min", "max"];

const __wbindgen_enum_GpuBufferBindingType = ["uniform", "storage", "read-only-storage"];

const __wbindgen_enum_GpuCanvasAlphaMode = ["opaque", "premultiplied"];

const __wbindgen_enum_GpuCompareFunction = ["never", "less", "equal", "less-equal", "greater", "not-equal", "greater-equal", "always"];

const __wbindgen_enum_GpuCullMode = ["none", "front", "back"];

const __wbindgen_enum_GpuDeviceLostReason = ["unknown", "destroyed"];

const __wbindgen_enum_GpuErrorFilter = ["validation", "out-of-memory", "internal"];

const __wbindgen_enum_GpuFilterMode = ["nearest", "linear"];

const __wbindgen_enum_GpuFrontFace = ["ccw", "cw"];

const __wbindgen_enum_GpuIndexFormat = ["uint16", "uint32"];

const __wbindgen_enum_GpuLoadOp = ["load", "clear"];

const __wbindgen_enum_GpuMipmapFilterMode = ["nearest", "linear"];

const __wbindgen_enum_GpuPowerPreference = ["low-power", "high-performance"];

const __wbindgen_enum_GpuPrimitiveTopology = ["point-list", "line-list", "line-strip", "triangle-list", "triangle-strip"];

const __wbindgen_enum_GpuQueryType = ["occlusion", "timestamp"];

const __wbindgen_enum_GpuSamplerBindingType = ["filtering", "non-filtering", "comparison"];

const __wbindgen_enum_GpuStencilOperation = ["keep", "zero", "replace", "invert", "increment-clamp", "decrement-clamp", "increment-wrap", "decrement-wrap"];

const __wbindgen_enum_GpuStorageTextureAccess = ["write-only", "read-only", "read-write"];

const __wbindgen_enum_GpuStoreOp = ["store", "discard"];

const __wbindgen_enum_GpuTextureAspect = ["all", "stencil-only", "depth-only"];

const __wbindgen_enum_GpuTextureDimension = ["1d", "2d", "3d"];

const __wbindgen_enum_GpuTextureFormat = ["r8unorm", "r8snorm", "r8uint", "r8sint", "r16uint", "r16sint", "r16float", "rg8unorm", "rg8snorm", "rg8uint", "rg8sint", "r32uint", "r32sint", "r32float", "rg16uint", "rg16sint", "rg16float", "rgba8unorm", "rgba8unorm-srgb", "rgba8snorm", "rgba8uint", "rgba8sint", "bgra8unorm", "bgra8unorm-srgb", "rgb9e5ufloat", "rgb10a2uint", "rgb10a2unorm", "rg11b10ufloat", "rg32uint", "rg32sint", "rg32float", "rgba16uint", "rgba16sint", "rgba16float", "rgba32uint", "rgba32sint", "rgba32float", "stencil8", "depth16unorm", "depth24plus", "depth24plus-stencil8", "depth32float", "depth32float-stencil8", "bc1-rgba-unorm", "bc1-rgba-unorm-srgb", "bc2-rgba-unorm", "bc2-rgba-unorm-srgb", "bc3-rgba-unorm", "bc3-rgba-unorm-srgb", "bc4-r-unorm", "bc4-r-snorm", "bc5-rg-unorm", "bc5-rg-snorm", "bc6h-rgb-ufloat", "bc6h-rgb-float", "bc7-rgba-unorm", "bc7-rgba-unorm-srgb", "etc2-rgb8unorm", "etc2-rgb8unorm-srgb", "etc2-rgb8a1unorm", "etc2-rgb8a1unorm-srgb", "etc2-rgba8unorm", "etc2-rgba8unorm-srgb", "eac-r11unorm", "eac-r11snorm", "eac-rg11unorm", "eac-rg11snorm", "astc-4x4-unorm", "astc-4x4-unorm-srgb", "astc-5x4-unorm", "astc-5x4-unorm-srgb", "astc-5x5-unorm", "astc-5x5-unorm-srgb", "astc-6x5-unorm", "astc-6x5-unorm-srgb", "astc-6x6-unorm", "astc-6x6-unorm-srgb", "astc-8x5-unorm", "astc-8x5-unorm-srgb", "astc-8x6-unorm", "astc-8x6-unorm-srgb", "astc-8x8-unorm", "astc-8x8-unorm-srgb", "astc-10x5-unorm", "astc-10x5-unorm-srgb", "astc-10x6-unorm", "astc-10x6-unorm-srgb", "astc-10x8-unorm", "astc-10x8-unorm-srgb", "astc-10x10-unorm", "astc-10x10-unorm-srgb", "astc-12x10-unorm", "astc-12x10-unorm-srgb", "astc-12x12-unorm", "astc-12x12-unorm-srgb"];

const __wbindgen_enum_GpuTextureSampleType = ["float", "unfilterable-float", "depth", "sint", "uint"];

const __wbindgen_enum_GpuTextureViewDimension = ["1d", "2d", "2d-array", "cube", "cube-array", "3d"];

const __wbindgen_enum_GpuVertexFormat = ["uint8", "uint8x2", "uint8x4", "sint8", "sint8x2", "sint8x4", "unorm8", "unorm8x2", "unorm8x4", "snorm8", "snorm8x2", "snorm8x4", "uint16", "uint16x2", "uint16x4", "sint16", "sint16x2", "sint16x4", "unorm16", "unorm16x2", "unorm16x4", "snorm16", "snorm16x2", "snorm16x4", "float16", "float16x2", "float16x4", "float32", "float32x2", "float32x3", "float32x4", "uint32", "uint32x2", "uint32x3", "uint32x4", "sint32", "sint32x2", "sint32x3", "sint32x4", "unorm10-10-10-2", "unorm8x4-bgra"];

const __wbindgen_enum_GpuVertexStepMode = ["vertex", "instance"];

const __wbindgen_enum_ResizeObserverBoxOptions = ["border-box", "content-box", "device-pixel-content-box"];

const __wbindgen_enum_VisibilityState = ["hidden", "visible"];

const CanvasFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_canvas_free(ptr >>> 0, 1));

const ColorFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_color_free(ptr >>> 0, 1));

export class Canvas {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        CanvasFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_canvas_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get playhead() {
        const ret = wasm.__wbg_get_canvas_playhead(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set playhead(arg0) {
        wasm.__wbg_set_canvas_playhead(this.__wbg_ptr, arg0);
    }
    enable_orbit_control() {
        wasm.canvas_enable_orbit_control(this.__wbg_ptr);
    }
    /**
     * @param {string} canvas_id
     */
    constructor(canvas_id) {
        const ptr0 = passStringToWasm0(canvas_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.canvas_new(ptr0, len0);
        this.__wbg_ptr = ret >>> 0;
        CanvasFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    run() {
        wasm.canvas_run(this.__wbg_ptr);
    }
}
if (Symbol.dispose) Canvas.prototype[Symbol.dispose] = Canvas.prototype.free;

export class Color {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Color.prototype);
        obj.__wbg_ptr = ptr;
        ColorFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ColorFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_color_free(ptr, 0);
    }
    /**
     * @param {string} hex_str
     * @returns {Color}
     */
    static from_hex_str(hex_str) {
        const ptr0 = passStringToWasm0(hex_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.color_from_hex_str(ptr0, len0);
        return Color.__wrap(ret);
    }
    /**
     * @param {number} r
     * @param {number} g
     * @param {number} b
     * @param {number} a
     */
    constructor(r, g, b, a) {
        const ret = wasm.color_new(r, g, b, a);
        this.__wbg_ptr = ret >>> 0;
        ColorFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
}
if (Symbol.dispose) Color.prototype[Symbol.dispose] = Color.prototype.free;

export function set_up() {
    wasm.set_up();
}

export function __wbg_Window_6419f7513544dd0b(arg0) {
    const ret = arg0.Window;
    return ret;
};

export function __wbg_Window_d1bf622f71ff0629(arg0) {
    const ret = arg0.Window;
    return ret;
};

export function __wbg_WorkerGlobalScope_147f18e856464ee4(arg0) {
    const ret = arg0.WorkerGlobalScope;
    return ret;
};

export function __wbg___wbindgen_debug_string_adfb662ae34724b6(arg0, arg1) {
    const ret = debugString(arg1);
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};

export function __wbg___wbindgen_is_function_8d400b8b1af978cd(arg0) {
    const ret = typeof(arg0) === 'function';
    return ret;
};

export function __wbg___wbindgen_is_null_dfda7d66506c95b5(arg0) {
    const ret = arg0 === null;
    return ret;
};

export function __wbg___wbindgen_is_object_ce774f3490692386(arg0) {
    const val = arg0;
    const ret = typeof(val) === 'object' && val !== null;
    return ret;
};

export function __wbg___wbindgen_is_undefined_f6b95eab589e0269(arg0) {
    const ret = arg0 === undefined;
    return ret;
};

export function __wbg___wbindgen_string_get_a2a31e16edf96e42(arg0, arg1) {
    const obj = arg1;
    const ret = typeof(obj) === 'string' ? obj : undefined;
    var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};

export function __wbg___wbindgen_throw_dd24417ed36fc46e(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

export function __wbg__wbg_cb_unref_87dfb5aaa0cbcea7(arg0) {
    arg0._wbg_cb_unref();
};

export function __wbg_abort_07646c894ebbf2bd(arg0) {
    arg0.abort();
};

export function __wbg_activeElement_b3e6b135325e4d5f(arg0) {
    const ret = arg0.activeElement;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};

export function __wbg_addEventListener_6a82629b3d430a48() { return handleError(function (arg0, arg1, arg2, arg3) {
    arg0.addEventListener(getStringFromWasm0(arg1, arg2), arg3);
}, arguments) };

export function __wbg_addListener_32ac5b9ed9d2a521() { return handleError(function (arg0, arg1) {
    arg0.addListener(arg1);
}, arguments) };

export function __wbg_altKey_56d1d642f3a28c92(arg0) {
    const ret = arg0.altKey;
    return ret;
};

export function __wbg_altKey_e13fae92dfebca3e(arg0) {
    const ret = arg0.altKey;
    return ret;
};

export function __wbg_animate_6ec571f163cf6f8d(arg0, arg1, arg2) {
    const ret = arg0.animate(arg1, arg2);
    return ret;
};

export function __wbg_appendChild_7465eba84213c75f() { return handleError(function (arg0, arg1) {
    const ret = arg0.appendChild(arg1);
    return ret;
}, arguments) };

export function __wbg_beginComputePass_d1fdb8126d3023c7(arg0, arg1) {
    const ret = arg0.beginComputePass(arg1);
    return ret;
};

export function __wbg_beginRenderPass_5959b1e03e4f545c() { return handleError(function (arg0, arg1) {
    const ret = arg0.beginRenderPass(arg1);
    return ret;
}, arguments) };

export function __wbg_blockSize_6456aaf09f0ab287(arg0) {
    const ret = arg0.blockSize;
    return ret;
};

export function __wbg_body_544738f8b03aef13(arg0) {
    const ret = arg0.body;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};

export function __wbg_brand_9562792cbb4735c3(arg0, arg1) {
    const ret = arg1.brand;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};

export function __wbg_brands_a1e7a2bce052128f(arg0) {
    const ret = arg0.brands;
    return ret;
};

export function __wbg_buffer_6cb2fecb1f253d71(arg0) {
    const ret = arg0.buffer;
    return ret;
};

export function __wbg_button_a54acd25bab5d442(arg0) {
    const ret = arg0.button;
    return ret;
};

export function __wbg_buttons_a37ff9ffacadddb5(arg0) {
    const ret = arg0.buttons;
    return ret;
};

export function __wbg_call_abb4ff46ce38be40() { return handleError(function (arg0, arg1) {
    const ret = arg0.call(arg1);
    return ret;
}, arguments) };

export function __wbg_cancelAnimationFrame_1c2a3faf7be5aedd() { return handleError(function (arg0, arg1) {
    arg0.cancelAnimationFrame(arg1);
}, arguments) };

export function __wbg_cancelIdleCallback_ee06eb3dcf335b86(arg0, arg1) {
    arg0.cancelIdleCallback(arg1 >>> 0);
};

export function __wbg_cancel_09c394f0894744eb(arg0) {
    arg0.cancel();
};

export function __wbg_catch_b9db41d97d42bd02(arg0, arg1) {
    const ret = arg0.catch(arg1);
    return ret;
};

export function __wbg_clearBuffer_2b0a3c8ac8b1cdab(arg0, arg1, arg2, arg3) {
    arg0.clearBuffer(arg1, arg2, arg3);
};

export function __wbg_clearBuffer_d734bcb0f4fad3c6(arg0, arg1, arg2) {
    arg0.clearBuffer(arg1, arg2);
};

export function __wbg_clearTimeout_1ca823b279705d35(arg0, arg1) {
    arg0.clearTimeout(arg1);
};

export function __wbg_close_8158530fc398ee2f(arg0) {
    arg0.close();
};

export function __wbg_code_b3ddfa90f724c486(arg0, arg1) {
    const ret = arg1.code;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};

export function __wbg_configure_8d74ee79dc392b1f() { return handleError(function (arg0, arg1) {
    arg0.configure(arg1);
}, arguments) };

export function __wbg_contains_457d2fc195838bfa(arg0, arg1) {
    const ret = arg0.contains(arg1);
    return ret;
};

export function __wbg_contentRect_1806147dfdc380d8(arg0) {
    const ret = arg0.contentRect;
    return ret;
};

export function __wbg_copyBufferToBuffer_8391faedae7bae2d() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
    arg0.copyBufferToBuffer(arg1, arg2, arg3, arg4);
}, arguments) };

export function __wbg_copyBufferToBuffer_db1c4fd94fdfa9a8() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5) {
    arg0.copyBufferToBuffer(arg1, arg2, arg3, arg4, arg5);
}, arguments) };

export function __wbg_copyBufferToTexture_c4bc464c7af9eb3d() { return handleError(function (arg0, arg1, arg2, arg3) {
    arg0.copyBufferToTexture(arg1, arg2, arg3);
}, arguments) };

export function __wbg_copyExternalImageToTexture_41327f54ff2be5fb() { return handleError(function (arg0, arg1, arg2, arg3) {
    arg0.copyExternalImageToTexture(arg1, arg2, arg3);
}, arguments) };

export function __wbg_copyTextureToBuffer_739b5accd0131afa() { return handleError(function (arg0, arg1, arg2, arg3) {
    arg0.copyTextureToBuffer(arg1, arg2, arg3);
}, arguments) };

export function __wbg_copyTextureToTexture_ecb35eeeccc84668() { return handleError(function (arg0, arg1, arg2, arg3) {
    arg0.copyTextureToTexture(arg1, arg2, arg3);
}, arguments) };

export function __wbg_createBindGroupLayout_37b290868edc95c3() { return handleError(function (arg0, arg1) {
    const ret = arg0.createBindGroupLayout(arg1);
    return ret;
}, arguments) };

export function __wbg_createBindGroup_9e48ec0df6021806(arg0, arg1) {
    const ret = arg0.createBindGroup(arg1);
    return ret;
};

export function __wbg_createBuffer_301327852bcb0fc9() { return handleError(function (arg0, arg1) {
    const ret = arg0.createBuffer(arg1);
    return ret;
}, arguments) };

export function __wbg_createCommandEncoder_f91fd6a7bbb31da6(arg0, arg1) {
    const ret = arg0.createCommandEncoder(arg1);
    return ret;
};

export function __wbg_createComputePipeline_63e73966ce7658ed(arg0, arg1) {
    const ret = arg0.createComputePipeline(arg1);
    return ret;
};

export function __wbg_createElement_da4ed2b219560fc6() { return handleError(function (arg0, arg1, arg2) {
    const ret = arg0.createElement(getStringFromWasm0(arg1, arg2));
    return ret;
}, arguments) };

export function __wbg_createObjectURL_7d9f7f8f41373850() { return handleError(function (arg0, arg1) {
    const ret = URL.createObjectURL(arg1);
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
}, arguments) };

export function __wbg_createPipelineLayout_e218679853a4ec90(arg0, arg1) {
    const ret = arg0.createPipelineLayout(arg1);
    return ret;
};

export function __wbg_createQuerySet_a263dc11313f1d4f() { return handleError(function (arg0, arg1) {
    const ret = arg0.createQuerySet(arg1);
    return ret;
}, arguments) };

export function __wbg_createRenderBundleEncoder_cc6623603aca6dcc() { return handleError(function (arg0, arg1) {
    const ret = arg0.createRenderBundleEncoder(arg1);
    return ret;
}, arguments) };

export function __wbg_createRenderPipeline_01226de8ac511c31() { return handleError(function (arg0, arg1) {
    const ret = arg0.createRenderPipeline(arg1);
    return ret;
}, arguments) };

export function __wbg_createSampler_dd08c9ffd5b1afa4(arg0, arg1) {
    const ret = arg0.createSampler(arg1);
    return ret;
};

export function __wbg_createShaderModule_a7e2ac8c2d5bd874(arg0, arg1) {
    const ret = arg0.createShaderModule(arg1);
    return ret;
};

export function __wbg_createTask_432d6d38dc688bee() { return handleError(function (arg0, arg1) {
    const ret = console.createTask(getStringFromWasm0(arg0, arg1));
    return ret;
}, arguments) };

export function __wbg_createTexture_47efd1fcfeeaeac8() { return handleError(function (arg0, arg1) {
    const ret = arg0.createTexture(arg1);
    return ret;
}, arguments) };

export function __wbg_createView_bb87ba5802a138dc() { return handleError(function (arg0, arg1) {
    const ret = arg0.createView(arg1);
    return ret;
}, arguments) };

export function __wbg_ctrlKey_487597b9069da036(arg0) {
    const ret = arg0.ctrlKey;
    return ret;
};

export function __wbg_ctrlKey_b391e5105c3f6e76(arg0) {
    const ret = arg0.ctrlKey;
    return ret;
};

export function __wbg_debug_9d0c87ddda3dc485(arg0) {
    console.debug(arg0);
};

export function __wbg_deltaMode_d74ec093e23ffeec(arg0) {
    const ret = arg0.deltaMode;
    return ret;
};

export function __wbg_deltaX_41f7678c94b10355(arg0) {
    const ret = arg0.deltaX;
    return ret;
};

export function __wbg_deltaY_3f10fd796fae2a0f(arg0) {
    const ret = arg0.deltaY;
    return ret;
};

export function __wbg_destroy_1fb0841289b41ab7(arg0) {
    arg0.destroy();
};

export function __wbg_destroy_511c665839f365c0(arg0) {
    arg0.destroy();
};

export function __wbg_destroy_c98dc18b3a071e98(arg0) {
    arg0.destroy();
};

export function __wbg_devicePixelContentBoxSize_4312b643ce19dcae(arg0) {
    const ret = arg0.devicePixelContentBoxSize;
    return ret;
};

export function __wbg_devicePixelRatio_390dee26c70aa30f(arg0) {
    const ret = arg0.devicePixelRatio;
    return ret;
};

export function __wbg_disconnect_0078fed2ab427a04(arg0) {
    arg0.disconnect();
};

export function __wbg_disconnect_94d44092a36f9880(arg0) {
    arg0.disconnect();
};

export function __wbg_document_5b745e82ba551ca5(arg0) {
    const ret = arg0.document;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};

export function __wbg_done_62ea16af4ce34b24(arg0) {
    const ret = arg0.done;
    return ret;
};

export function __wbg_drawIndexedIndirect_42fe3c5b17fdc555(arg0, arg1, arg2) {
    arg0.drawIndexedIndirect(arg1, arg2);
};

export function __wbg_drawIndexed_3cb778da4c5793f5(arg0, arg1, arg2, arg3, arg4, arg5) {
    arg0.drawIndexed(arg1 >>> 0, arg2 >>> 0, arg3 >>> 0, arg4, arg5 >>> 0);
};

export function __wbg_drawIndirect_549f56d168b141b3(arg0, arg1, arg2) {
    arg0.drawIndirect(arg1, arg2);
};

export function __wbg_draw_35bd445973b180dc(arg0, arg1, arg2, arg3, arg4) {
    arg0.draw(arg1 >>> 0, arg2 >>> 0, arg3 >>> 0, arg4 >>> 0);
};

export function __wbg_end_ddc7a483fce32eed(arg0) {
    arg0.end();
};

export function __wbg_error_1a829178de44fe4e(arg0) {
    const ret = arg0.error;
    return ret;
};

export function __wbg_error_7534b8e9a36f1ab4(arg0, arg1) {
    let deferred0_0;
    let deferred0_1;
    try {
        deferred0_0 = arg0;
        deferred0_1 = arg1;
        console.error(getStringFromWasm0(arg0, arg1));
    } finally {
        wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
    }
};

export function __wbg_error_7bc7d576a6aaf855(arg0) {
    console.error(arg0);
};

export function __wbg_error_d7f117185d9ffd19(arg0, arg1) {
    console.error(arg0, arg1);
};

export function __wbg_executeBundles_84e1e9326fd29d93(arg0, arg1) {
    arg0.executeBundles(arg1);
};

export function __wbg_exitFullscreen_14c765e2bd192c7b(arg0) {
    arg0.exitFullscreen();
};

export function __wbg_features_7463d4000d7c57a2(arg0) {
    const ret = arg0.features;
    return ret;
};

export function __wbg_features_dafff7dd39a9b665(arg0) {
    const ret = arg0.features;
    return ret;
};

export function __wbg_finish_7c3e136077cc2230(arg0) {
    const ret = arg0.finish();
    return ret;
};

export function __wbg_finish_db51f74029254467(arg0, arg1) {
    const ret = arg0.finish(arg1);
    return ret;
};

export function __wbg_focus_220a53e22147dc0f() { return handleError(function (arg0) {
    arg0.focus();
}, arguments) };

export function __wbg_fullscreenElement_e2e939644adf50e1(arg0) {
    const ret = arg0.fullscreenElement;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};

export function __wbg_getCoalescedEvents_21492912fd0145ec(arg0) {
    const ret = arg0.getCoalescedEvents;
    return ret;
};

export function __wbg_getCoalescedEvents_43b8965761bb13ef(arg0) {
    const ret = arg0.getCoalescedEvents();
    return ret;
};

export function __wbg_getComputedStyle_bbcd5e3d08077b71() { return handleError(function (arg0, arg1) {
    const ret = arg0.getComputedStyle(arg1);
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
}, arguments) };

export function __wbg_getContext_01f42b234e833f0a() { return handleError(function (arg0, arg1, arg2) {
    const ret = arg0.getContext(getStringFromWasm0(arg1, arg2));
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
}, arguments) };

export function __wbg_getContext_2f210d0a58d43d95() { return handleError(function (arg0, arg1, arg2) {
    const ret = arg0.getContext(getStringFromWasm0(arg1, arg2));
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
}, arguments) };

export function __wbg_getCurrentTexture_b82524d31095411f() { return handleError(function (arg0) {
    const ret = arg0.getCurrentTexture();
    return ret;
}, arguments) };

export function __wbg_getElementById_e05488d2143c2b21(arg0, arg1, arg2) {
    const ret = arg0.getElementById(getStringFromWasm0(arg1, arg2));
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};

export function __wbg_getMappedRange_98acf7ad62c501ee() { return handleError(function (arg0, arg1, arg2) {
    const ret = arg0.getMappedRange(arg1, arg2);
    return ret;
}, arguments) };

export function __wbg_getOwnPropertyDescriptor_b6aa5a2fa50d52c7(arg0, arg1) {
    const ret = Object.getOwnPropertyDescriptor(arg0, arg1);
    return ret;
};

export function __wbg_getPreferredCanvasFormat_92cc631581256e43(arg0) {
    const ret = arg0.getPreferredCanvasFormat();
    return (__wbindgen_enum_GpuTextureFormat.indexOf(ret) + 1 || 96) - 1;
};

export function __wbg_getPropertyValue_dcded91357966805() { return handleError(function (arg0, arg1, arg2, arg3) {
    const ret = arg1.getPropertyValue(getStringFromWasm0(arg2, arg3));
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
}, arguments) };

export function __wbg_get_6b7bd52aca3f9671(arg0, arg1) {
    const ret = arg0[arg1 >>> 0];
    return ret;
};

export function __wbg_get_c53d381635aa3929(arg0, arg1) {
    const ret = arg0[arg1 >>> 0];
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};

export function __wbg_gpu_4b2187814fd587ca(arg0) {
    const ret = arg0.gpu;
    return ret;
};

export function __wbg_has_e7b9469a0ae9abd2(arg0, arg1, arg2) {
    const ret = arg0.has(getStringFromWasm0(arg1, arg2));
    return ret;
};

export function __wbg_height_5d22b94a936fae9f(arg0) {
    const ret = arg0.height;
    return ret;
};

export function __wbg_info_ce6bcc489c22f6f0(arg0) {
    console.info(arg0);
};

export function __wbg_inlineSize_65c8cd0ecc54c605(arg0) {
    const ret = arg0.inlineSize;
    return ret;
};

export function __wbg_instanceof_GpuAdapter_5e451ad6596e2784(arg0) {
    let result;
    try {
        result = arg0 instanceof GPUAdapter;
    } catch (_) {
        result = false;
    }
    const ret = result;
    return ret;
};

export function __wbg_instanceof_GpuCanvasContext_f70ee27f49f4f884(arg0) {
    let result;
    try {
        result = arg0 instanceof GPUCanvasContext;
    } catch (_) {
        result = false;
    }
    const ret = result;
    return ret;
};

export function __wbg_instanceof_GpuDeviceLostInfo_2060b770b1a9a12f(arg0) {
    let result;
    try {
        result = arg0 instanceof GPUDeviceLostInfo;
    } catch (_) {
        result = false;
    }
    const ret = result;
    return ret;
};

export function __wbg_instanceof_GpuOutOfMemoryError_d312fd1714771dbd(arg0) {
    let result;
    try {
        result = arg0 instanceof GPUOutOfMemoryError;
    } catch (_) {
        result = false;
    }
    const ret = result;
    return ret;
};

export function __wbg_instanceof_GpuValidationError_eb3c494ad7b55611(arg0) {
    let result;
    try {
        result = arg0 instanceof GPUValidationError;
    } catch (_) {
        result = false;
    }
    const ret = result;
    return ret;
};

export function __wbg_instanceof_Object_577e21051f7bcb79(arg0) {
    let result;
    try {
        result = arg0 instanceof Object;
    } catch (_) {
        result = false;
    }
    const ret = result;
    return ret;
};

export function __wbg_instanceof_Window_b5cf7783caa68180(arg0) {
    let result;
    try {
        result = arg0 instanceof Window;
    } catch (_) {
        result = false;
    }
    const ret = result;
    return ret;
};

export function __wbg_isIntersecting_2d00a342ea420fb9(arg0) {
    const ret = arg0.isIntersecting;
    return ret;
};

export function __wbg_is_928aa29d71e75457(arg0, arg1) {
    const ret = Object.is(arg0, arg1);
    return ret;
};

export function __wbg_key_505d33c50799526a(arg0, arg1) {
    const ret = arg1.key;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};

export function __wbg_keys_af2028954708892b(arg0) {
    const ret = arg0.keys();
    return ret;
};

export function __wbg_label_8296b38115112ca4(arg0, arg1) {
    const ret = arg1.label;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};

export function __wbg_length_22ac23eaec9d8053(arg0) {
    const ret = arg0.length;
    return ret;
};

export function __wbg_length_d45040a40c570362(arg0) {
    const ret = arg0.length;
    return ret;
};

export function __wbg_limits_22116faf3a912173(arg0) {
    const ret = arg0.limits;
    return ret;
};

export function __wbg_limits_b79b8275a12805b2(arg0) {
    const ret = arg0.limits;
    return ret;
};

export function __wbg_location_0ef648bbeb3e599c(arg0) {
    const ret = arg0.location;
    return ret;
};

export function __wbg_log_1d990106d99dacb7(arg0) {
    console.log(arg0);
};

export function __wbg_lost_127bd218dad158f4(arg0) {
    const ret = arg0.lost;
    return ret;
};

export function __wbg_mapAsync_2dba5c7b48d2e598(arg0, arg1, arg2, arg3) {
    const ret = arg0.mapAsync(arg1 >>> 0, arg2, arg3);
    return ret;
};

export function __wbg_matchMedia_29904c79dbaba90b() { return handleError(function (arg0, arg1, arg2) {
    const ret = arg0.matchMedia(getStringFromWasm0(arg1, arg2));
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
}, arguments) };

export function __wbg_matches_9cef9b7c722bd7c8(arg0) {
    const ret = arg0.matches;
    return ret;
};

export function __wbg_maxBindGroups_af2c64a371bc64b2(arg0) {
    const ret = arg0.maxBindGroups;
    return ret;
};

export function __wbg_maxBindingsPerBindGroup_430f6510523172d9(arg0) {
    const ret = arg0.maxBindingsPerBindGroup;
    return ret;
};

export function __wbg_maxBufferSize_68b45c1b69c22207(arg0) {
    const ret = arg0.maxBufferSize;
    return ret;
};

export function __wbg_maxColorAttachmentBytesPerSample_cbfce6f5737b4853(arg0) {
    const ret = arg0.maxColorAttachmentBytesPerSample;
    return ret;
};

export function __wbg_maxColorAttachments_70e7c33a58d9fc56(arg0) {
    const ret = arg0.maxColorAttachments;
    return ret;
};

export function __wbg_maxComputeInvocationsPerWorkgroup_4ad21bf35b7bd17f(arg0) {
    const ret = arg0.maxComputeInvocationsPerWorkgroup;
    return ret;
};

export function __wbg_maxComputeWorkgroupSizeX_854c87a3ea2e5a00(arg0) {
    const ret = arg0.maxComputeWorkgroupSizeX;
    return ret;
};

export function __wbg_maxComputeWorkgroupSizeY_965ebcb7fee4acf5(arg0) {
    const ret = arg0.maxComputeWorkgroupSizeY;
    return ret;
};

export function __wbg_maxComputeWorkgroupSizeZ_3bf468106936874c(arg0) {
    const ret = arg0.maxComputeWorkgroupSizeZ;
    return ret;
};

export function __wbg_maxComputeWorkgroupStorageSize_b9cab4f75b0f03e3(arg0) {
    const ret = arg0.maxComputeWorkgroupStorageSize;
    return ret;
};

export function __wbg_maxComputeWorkgroupsPerDimension_f4664066d76015da(arg0) {
    const ret = arg0.maxComputeWorkgroupsPerDimension;
    return ret;
};

export function __wbg_maxDynamicStorageBuffersPerPipelineLayout_6b7faf56a6e328ad(arg0) {
    const ret = arg0.maxDynamicStorageBuffersPerPipelineLayout;
    return ret;
};

export function __wbg_maxDynamicUniformBuffersPerPipelineLayout_22a38cc27e2f4626(arg0) {
    const ret = arg0.maxDynamicUniformBuffersPerPipelineLayout;
    return ret;
};

export function __wbg_maxSampledTexturesPerShaderStage_97c70c39fb197a2b(arg0) {
    const ret = arg0.maxSampledTexturesPerShaderStage;
    return ret;
};

export function __wbg_maxSamplersPerShaderStage_a148c7e536a3807c(arg0) {
    const ret = arg0.maxSamplersPerShaderStage;
    return ret;
};

export function __wbg_maxStorageBufferBindingSize_bfaa9c302ad157e3(arg0) {
    const ret = arg0.maxStorageBufferBindingSize;
    return ret;
};

export function __wbg_maxStorageBuffersPerShaderStage_463d04005d78f248(arg0) {
    const ret = arg0.maxStorageBuffersPerShaderStage;
    return ret;
};

export function __wbg_maxStorageTexturesPerShaderStage_3fe774bbe6ad1371(arg0) {
    const ret = arg0.maxStorageTexturesPerShaderStage;
    return ret;
};

export function __wbg_maxTextureArrayLayers_6b1a7b0b3b4c0556(arg0) {
    const ret = arg0.maxTextureArrayLayers;
    return ret;
};

export function __wbg_maxTextureDimension1D_e79117695a706815(arg0) {
    const ret = arg0.maxTextureDimension1D;
    return ret;
};

export function __wbg_maxTextureDimension2D_cbb3e7343bea93d1(arg0) {
    const ret = arg0.maxTextureDimension2D;
    return ret;
};

export function __wbg_maxTextureDimension3D_7ac996fb8fe18286(arg0) {
    const ret = arg0.maxTextureDimension3D;
    return ret;
};

export function __wbg_maxUniformBufferBindingSize_22c4f55b73d306cf(arg0) {
    const ret = arg0.maxUniformBufferBindingSize;
    return ret;
};

export function __wbg_maxUniformBuffersPerShaderStage_65e2b2eaf78ef4e1(arg0) {
    const ret = arg0.maxUniformBuffersPerShaderStage;
    return ret;
};

export function __wbg_maxVertexAttributes_a6c97c2dc4a8d443(arg0) {
    const ret = arg0.maxVertexAttributes;
    return ret;
};

export function __wbg_maxVertexBufferArrayStride_305ba73c4de05f82(arg0) {
    const ret = arg0.maxVertexBufferArrayStride;
    return ret;
};

export function __wbg_maxVertexBuffers_df4a4911d2c540d8(arg0) {
    const ret = arg0.maxVertexBuffers;
    return ret;
};

export function __wbg_media_077ecdcd98f5aa28(arg0, arg1) {
    const ret = arg1.media;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};

export function __wbg_message_7bd11486f13d13ab(arg0, arg1) {
    const ret = arg1.message;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};

export function __wbg_message_ed58662d040ec0c0(arg0, arg1) {
    const ret = arg1.message;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};

export function __wbg_metaKey_0572b1cbcb5b272b(arg0) {
    const ret = arg0.metaKey;
    return ret;
};

export function __wbg_metaKey_448c751accad2eba(arg0) {
    const ret = arg0.metaKey;
    return ret;
};

export function __wbg_minStorageBufferOffsetAlignment_12d731adbf75fd21(arg0) {
    const ret = arg0.minStorageBufferOffsetAlignment;
    return ret;
};

export function __wbg_minUniformBufferOffsetAlignment_2a0a0d2e84c280a7(arg0) {
    const ret = arg0.minUniformBufferOffsetAlignment;
    return ret;
};

export function __wbg_movementX_00c85de14e45c5f4(arg0) {
    const ret = arg0.movementX;
    return ret;
};

export function __wbg_movementY_9f8470917a12f3f5(arg0) {
    const ret = arg0.movementY;
    return ret;
};

export function __wbg_navigator_11b7299bb7886507(arg0) {
    const ret = arg0.navigator;
    return ret;
};

export function __wbg_navigator_b49edef831236138(arg0) {
    const ret = arg0.navigator;
    return ret;
};

export function __wbg_new_137453588c393c59() { return handleError(function () {
    const ret = new MessageChannel();
    return ret;
}, arguments) };

export function __wbg_new_1ba21ce319a06297() {
    const ret = new Object();
    return ret;
};

export function __wbg_new_25f239778d6112b9() {
    const ret = new Array();
    return ret;
};

export function __wbg_new_53cb1e86c1ef5d2a() { return handleError(function (arg0, arg1) {
    const ret = new Worker(getStringFromWasm0(arg0, arg1));
    return ret;
}, arguments) };

export function __wbg_new_881a222c65f168fc() { return handleError(function () {
    const ret = new AbortController();
    return ret;
}, arguments) };

export function __wbg_new_8a6f238a6ece86ea() {
    const ret = new Error();
    return ret;
};

export function __wbg_new_a25bd305a87faf63() { return handleError(function (arg0) {
    const ret = new ResizeObserver(arg0);
    return ret;
}, arguments) };

export function __wbg_new_bba60878a7b7f42c() { return handleError(function (arg0) {
    const ret = new IntersectionObserver(arg0);
    return ret;
}, arguments) };

export function __wbg_new_from_slice_f9c22b9153b26992(arg0, arg1) {
    const ret = new Uint8Array(getArrayU8FromWasm0(arg0, arg1));
    return ret;
};

export function __wbg_new_no_args_cb138f77cf6151ee(arg0, arg1) {
    const ret = new Function(getStringFromWasm0(arg0, arg1));
    return ret;
};

export function __wbg_new_with_byte_offset_and_length_d85c3da1fd8df149(arg0, arg1, arg2) {
    const ret = new Uint8Array(arg0, arg1 >>> 0, arg2 >>> 0);
    return ret;
};

export function __wbg_new_with_str_sequence_and_options_fe06fc75a8482fd3() { return handleError(function (arg0, arg1) {
    const ret = new Blob(arg0, arg1);
    return ret;
}, arguments) };

export function __wbg_next_3cfe5c0fe2a4cc53() { return handleError(function (arg0) {
    const ret = arg0.next();
    return ret;
}, arguments) };

export function __wbg_now_2c95c9de01293173(arg0) {
    const ret = arg0.now();
    return ret;
};

export function __wbg_observe_5186b67ce86740f9(arg0, arg1) {
    arg0.observe(arg1);
};

export function __wbg_observe_ce343c3f1701b1f1(arg0, arg1, arg2) {
    arg0.observe(arg1, arg2);
};

export function __wbg_observe_eefa2465578e5d51(arg0, arg1) {
    arg0.observe(arg1);
};

export function __wbg_of_6505a0eb509da02e(arg0) {
    const ret = Array.of(arg0);
    return ret;
};

export function __wbg_of_b8cd42ebb79fb759(arg0, arg1) {
    const ret = Array.of(arg0, arg1);
    return ret;
};

export function __wbg_offsetX_cb6a38e6f23cb4a6(arg0) {
    const ret = arg0.offsetX;
    return ret;
};

export function __wbg_offsetY_43e21941c5c1f8bf(arg0) {
    const ret = arg0.offsetY;
    return ret;
};

export function __wbg_onSubmittedWorkDone_22f709e16b81d1c2(arg0) {
    const ret = arg0.onSubmittedWorkDone();
    return ret;
};

export function __wbg_performance_7a3ffd0b17f663ad(arg0) {
    const ret = arg0.performance;
    return ret;
};

export function __wbg_persisted_90586ee41f1f0188(arg0) {
    const ret = arg0.persisted;
    return ret;
};

export function __wbg_play_63bc12f42e16af91(arg0) {
    arg0.play();
};

export function __wbg_pointerId_bf4326e151df1474(arg0) {
    const ret = arg0.pointerId;
    return ret;
};

export function __wbg_pointerType_f1939c6407f96be9(arg0, arg1) {
    const ret = arg1.pointerType;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};

export function __wbg_popErrorScope_3620d0770e0c967f(arg0) {
    const ret = arg0.popErrorScope();
    return ret;
};

export function __wbg_port1_75dce9d0d8087125(arg0) {
    const ret = arg0.port1;
    return ret;
};

export function __wbg_port2_3cffa4119380f41d(arg0) {
    const ret = arg0.port2;
    return ret;
};

export function __wbg_postMessage_79f844174f56304f() { return handleError(function (arg0, arg1) {
    arg0.postMessage(arg1);
}, arguments) };

export function __wbg_postMessage_e0309b53c7ad30e6() { return handleError(function (arg0, arg1, arg2) {
    arg0.postMessage(arg1, arg2);
}, arguments) };

export function __wbg_postTask_41d93e93941e4a3d(arg0, arg1, arg2) {
    const ret = arg0.postTask(arg1, arg2);
    return ret;
};

export function __wbg_pressure_35422752c1a40439(arg0) {
    const ret = arg0.pressure;
    return ret;
};

export function __wbg_preventDefault_e97663aeeb9709d3(arg0) {
    arg0.preventDefault();
};

export function __wbg_prototype_c28bca39c45aba9b() {
    const ret = ResizeObserverEntry.prototype;
    return ret;
};

export function __wbg_prototypesetcall_dfe9b766cdc1f1fd(arg0, arg1, arg2) {
    Uint8Array.prototype.set.call(getArrayU8FromWasm0(arg0, arg1), arg2);
};

export function __wbg_pushErrorScope_82cb69cc547ce5fb(arg0, arg1) {
    arg0.pushErrorScope(__wbindgen_enum_GpuErrorFilter[arg1]);
};

export function __wbg_push_7d9be8f38fc13975(arg0, arg1) {
    const ret = arg0.push(arg1);
    return ret;
};

export function __wbg_querySelectorAll_aa1048eae18f6f1a() { return handleError(function (arg0, arg1, arg2) {
    const ret = arg0.querySelectorAll(getStringFromWasm0(arg1, arg2));
    return ret;
}, arguments) };

export function __wbg_queueMicrotask_892c6bd5d40fe78e(arg0, arg1) {
    arg0.queueMicrotask(arg1);
};

export function __wbg_queueMicrotask_9b549dfce8865860(arg0) {
    const ret = arg0.queueMicrotask;
    return ret;
};

export function __wbg_queueMicrotask_fca69f5bfad613a5(arg0) {
    queueMicrotask(arg0);
};

export function __wbg_queue_e7ab52ab0880dce9(arg0) {
    const ret = arg0.queue;
    return ret;
};

export function __wbg_reason_92874ec807ec200c(arg0) {
    const ret = arg0.reason;
    return (__wbindgen_enum_GpuDeviceLostReason.indexOf(ret) + 1 || 3) - 1;
};

export function __wbg_removeEventListener_565e273024b68b75() { return handleError(function (arg0, arg1, arg2, arg3) {
    arg0.removeEventListener(getStringFromWasm0(arg1, arg2), arg3);
}, arguments) };

export function __wbg_removeListener_204002d1eb3f20f6() { return handleError(function (arg0, arg1) {
    arg0.removeListener(arg1);
}, arguments) };

export function __wbg_removeProperty_c2e16faee2834bef() { return handleError(function (arg0, arg1, arg2, arg3) {
    const ret = arg1.removeProperty(getStringFromWasm0(arg2, arg3));
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
}, arguments) };

export function __wbg_repeat_3733d1d584bf0e38(arg0) {
    const ret = arg0.repeat;
    return ret;
};

export function __wbg_requestAdapter_eb00393b717ebb9c(arg0, arg1) {
    const ret = arg0.requestAdapter(arg1);
    return ret;
};

export function __wbg_requestAnimationFrame_994dc4ebde22b8d9() { return handleError(function (arg0, arg1) {
    const ret = arg0.requestAnimationFrame(arg1);
    return ret;
}, arguments) };

export function __wbg_requestDevice_1be6e30ff9d67933(arg0, arg1) {
    const ret = arg0.requestDevice(arg1);
    return ret;
};

export function __wbg_requestFullscreen_86fc6cdb76000482(arg0) {
    const ret = arg0.requestFullscreen;
    return ret;
};

export function __wbg_requestFullscreen_9f0611438eb929cf(arg0) {
    const ret = arg0.requestFullscreen();
    return ret;
};

export function __wbg_requestIdleCallback_1b8d644ff564208f(arg0) {
    const ret = arg0.requestIdleCallback;
    return ret;
};

export function __wbg_requestIdleCallback_dedd367f2e61f932() { return handleError(function (arg0, arg1) {
    const ret = arg0.requestIdleCallback(arg1);
    return ret;
}, arguments) };

export function __wbg_resolveQuerySet_44dddc4a814652f2(arg0, arg1, arg2, arg3, arg4, arg5) {
    arg0.resolveQuerySet(arg1, arg2 >>> 0, arg3 >>> 0, arg4, arg5 >>> 0);
};

export function __wbg_resolve_fd5bfbaa4ce36e1e(arg0) {
    const ret = Promise.resolve(arg0);
    return ret;
};

export function __wbg_revokeObjectURL_88db3468842ff09e() { return handleError(function (arg0, arg1) {
    URL.revokeObjectURL(getStringFromWasm0(arg0, arg1));
}, arguments) };

export function __wbg_run_51bf644e39739ca6(arg0, arg1, arg2) {
    try {
        var state0 = {a: arg1, b: arg2};
        var cb0 = () => {
            const a = state0.a;
            state0.a = 0;
            try {
                return wasm_bindgen__convert__closures_____invoke__hd9dfa106a4aaa8b7(a, state0.b, );
            } finally {
                state0.a = a;
            }
        };
        const ret = arg0.run(cb0);
        return ret;
    } finally {
        state0.a = state0.b = 0;
    }
};

export function __wbg_scheduler_48482a9974eeacbd(arg0) {
    const ret = arg0.scheduler;
    return ret;
};

export function __wbg_scheduler_5156bb61cc1cf589(arg0) {
    const ret = arg0.scheduler;
    return ret;
};

export function __wbg_setAttribute_34747dd193f45828() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
    arg0.setAttribute(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
}, arguments) };

export function __wbg_setBindGroup_0ae63a01a1ed4c73(arg0, arg1, arg2) {
    arg0.setBindGroup(arg1 >>> 0, arg2);
};

export function __wbg_setBindGroup_d906e4c5d8533957() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
    arg0.setBindGroup(arg1 >>> 0, arg2, getArrayU32FromWasm0(arg3, arg4), arg5, arg6 >>> 0);
}, arguments) };

export function __wbg_setBlendConstant_35937accbe201fdd() { return handleError(function (arg0, arg1) {
    arg0.setBlendConstant(arg1);
}, arguments) };

export function __wbg_setIndexBuffer_c7ecba3588b25ce2(arg0, arg1, arg2, arg3) {
    arg0.setIndexBuffer(arg1, __wbindgen_enum_GpuIndexFormat[arg2], arg3);
};

export function __wbg_setIndexBuffer_db41507e5114fad4(arg0, arg1, arg2, arg3, arg4) {
    arg0.setIndexBuffer(arg1, __wbindgen_enum_GpuIndexFormat[arg2], arg3, arg4);
};

export function __wbg_setPipeline_b010841b1ab020c5(arg0, arg1) {
    arg0.setPipeline(arg1);
};

export function __wbg_setPointerCapture_c611f4bcb7e9081e() { return handleError(function (arg0, arg1) {
    arg0.setPointerCapture(arg1);
}, arguments) };

export function __wbg_setProperty_f27b2c05323daf8a() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
    arg0.setProperty(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
}, arguments) };

export function __wbg_setScissorRect_48aad86f2b04be65(arg0, arg1, arg2, arg3, arg4) {
    arg0.setScissorRect(arg1 >>> 0, arg2 >>> 0, arg3 >>> 0, arg4 >>> 0);
};

export function __wbg_setStencilReference_0193bdfe3e999b05(arg0, arg1) {
    arg0.setStencilReference(arg1 >>> 0);
};

export function __wbg_setTimeout_06477c23d31efef1() { return handleError(function (arg0, arg1, arg2) {
    const ret = arg0.setTimeout(arg1, arg2);
    return ret;
}, arguments) };

export function __wbg_setTimeout_780045617e4bd6d6() { return handleError(function (arg0, arg1) {
    const ret = arg0.setTimeout(arg1);
    return ret;
}, arguments) };

export function __wbg_setVertexBuffer_da6ef21c06e9c5ac(arg0, arg1, arg2, arg3, arg4) {
    arg0.setVertexBuffer(arg1 >>> 0, arg2, arg3, arg4);
};

export function __wbg_setVertexBuffer_f209d2bcc82ece37(arg0, arg1, arg2, arg3) {
    arg0.setVertexBuffer(arg1 >>> 0, arg2, arg3);
};

export function __wbg_setViewport_bee857cbfc17f5bf(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
    arg0.setViewport(arg1, arg2, arg3, arg4, arg5, arg6);
};

export function __wbg_set_781438a03c0c3c81() { return handleError(function (arg0, arg1, arg2) {
    const ret = Reflect.set(arg0, arg1, arg2);
    return ret;
}, arguments) };

export function __wbg_set_a_004bf5b9918b7a9d(arg0, arg1) {
    arg0.a = arg1;
};

export function __wbg_set_access_615d472480b556e8(arg0, arg1) {
    arg0.access = __wbindgen_enum_GpuStorageTextureAccess[arg1];
};

export function __wbg_set_address_mode_u_f8c82bdfe28ff814(arg0, arg1) {
    arg0.addressModeU = __wbindgen_enum_GpuAddressMode[arg1];
};

export function __wbg_set_address_mode_v_15cc0a4331c8a793(arg0, arg1) {
    arg0.addressModeV = __wbindgen_enum_GpuAddressMode[arg1];
};

export function __wbg_set_address_mode_w_b3ede4a69eef8df8(arg0, arg1) {
    arg0.addressModeW = __wbindgen_enum_GpuAddressMode[arg1];
};

export function __wbg_set_alpha_7c9ec1b9552caf33(arg0, arg1) {
    arg0.alpha = arg1;
};

export function __wbg_set_alpha_mode_d776091480150822(arg0, arg1) {
    arg0.alphaMode = __wbindgen_enum_GpuCanvasAlphaMode[arg1];
};

export function __wbg_set_alpha_to_coverage_enabled_97c65e8e0f0f97f0(arg0, arg1) {
    arg0.alphaToCoverageEnabled = arg1 !== 0;
};

export function __wbg_set_array_layer_count_4b8708bd126ac758(arg0, arg1) {
    arg0.arrayLayerCount = arg1 >>> 0;
};

export function __wbg_set_array_stride_89addb9ef89545a3(arg0, arg1) {
    arg0.arrayStride = arg1;
};

export function __wbg_set_aspect_e672528231f771cb(arg0, arg1) {
    arg0.aspect = __wbindgen_enum_GpuTextureAspect[arg1];
};

export function __wbg_set_aspect_f5c27f8e9589644d(arg0, arg1) {
    arg0.aspect = __wbindgen_enum_GpuTextureAspect[arg1];
};

export function __wbg_set_attributes_2ab28c57eed0dc3a(arg0, arg1) {
    arg0.attributes = arg1;
};

export function __wbg_set_b_b2b86286be8253f1(arg0, arg1) {
    arg0.b = arg1;
};

export function __wbg_set_base_array_layer_a3268c17b424196f(arg0, arg1) {
    arg0.baseArrayLayer = arg1 >>> 0;
};

export function __wbg_set_base_mip_level_7ac60a20e24c81b1(arg0, arg1) {
    arg0.baseMipLevel = arg1 >>> 0;
};

export function __wbg_set_bc3a432bdcd60886(arg0, arg1, arg2) {
    arg0.set(arg1, arg2 >>> 0);
};

export function __wbg_set_beginning_of_pass_write_index_2de01bde51c7b0c4(arg0, arg1) {
    arg0.beginningOfPassWriteIndex = arg1 >>> 0;
};

export function __wbg_set_beginning_of_pass_write_index_87e36fb6887d3c1c(arg0, arg1) {
    arg0.beginningOfPassWriteIndex = arg1 >>> 0;
};

export function __wbg_set_bind_group_layouts_7fedf360e81319eb(arg0, arg1) {
    arg0.bindGroupLayouts = arg1;
};

export function __wbg_set_binding_030f427cbe0e3a55(arg0, arg1) {
    arg0.binding = arg1 >>> 0;
};

export function __wbg_set_binding_69fdec34b16b327b(arg0, arg1) {
    arg0.binding = arg1 >>> 0;
};

export function __wbg_set_blend_c6896375c7f0119c(arg0, arg1) {
    arg0.blend = arg1;
};

export function __wbg_set_box_d724bbbe6354cf86(arg0, arg1) {
    arg0.box = __wbindgen_enum_ResizeObserverBoxOptions[arg1];
};

export function __wbg_set_buffer_b70ef3f40d503e25(arg0, arg1) {
    arg0.buffer = arg1;
};

export function __wbg_set_buffer_b79f2efcb24ba844(arg0, arg1) {
    arg0.buffer = arg1;
};

export function __wbg_set_buffer_c23b131bfa95f222(arg0, arg1) {
    arg0.buffer = arg1;
};

export function __wbg_set_buffers_14ec06929ea541ec(arg0, arg1) {
    arg0.buffers = arg1;
};

export function __wbg_set_bytes_per_row_279f81f686787a9f(arg0, arg1) {
    arg0.bytesPerRow = arg1 >>> 0;
};

export function __wbg_set_bytes_per_row_fbb55671d2ba86f2(arg0, arg1) {
    arg0.bytesPerRow = arg1 >>> 0;
};

export function __wbg_set_clear_value_829dfd0db30aaeac(arg0, arg1) {
    arg0.clearValue = arg1;
};

export function __wbg_set_code_09748e5373b711b2(arg0, arg1, arg2) {
    arg0.code = getStringFromWasm0(arg1, arg2);
};

export function __wbg_set_color_96b2f28b4f51fceb(arg0, arg1) {
    arg0.color = arg1;
};

export function __wbg_set_color_attachments_ee51f860224ee6dd(arg0, arg1) {
    arg0.colorAttachments = arg1;
};

export function __wbg_set_color_formats_1ab6364cf6d288e9(arg0, arg1) {
    arg0.colorFormats = arg1;
};

export function __wbg_set_compare_61125878543846d0(arg0, arg1) {
    arg0.compare = __wbindgen_enum_GpuCompareFunction[arg1];
};

export function __wbg_set_compare_eb86f2890782b20b(arg0, arg1) {
    arg0.compare = __wbindgen_enum_GpuCompareFunction[arg1];
};

export function __wbg_set_compute_e2902436ce2ed757(arg0, arg1) {
    arg0.compute = arg1;
};

export function __wbg_set_count_4d43f3f3ab7f952d(arg0, arg1) {
    arg0.count = arg1 >>> 0;
};

export function __wbg_set_count_c555ce929443aa66(arg0, arg1) {
    arg0.count = arg1 >>> 0;
};

export function __wbg_set_cull_mode_4e0bb3799474c091(arg0, arg1) {
    arg0.cullMode = __wbindgen_enum_GpuCullMode[arg1];
};

export function __wbg_set_depth_bias_clamp_5375d337b8b35cd8(arg0, arg1) {
    arg0.depthBiasClamp = arg1;
};

export function __wbg_set_depth_bias_ea8b79f02442c9c7(arg0, arg1) {
    arg0.depthBias = arg1;
};

export function __wbg_set_depth_bias_slope_scale_0493feedbe6ad438(arg0, arg1) {
    arg0.depthBiasSlopeScale = arg1;
};

export function __wbg_set_depth_clear_value_20534499c6507e19(arg0, arg1) {
    arg0.depthClearValue = arg1;
};

export function __wbg_set_depth_compare_00e8b65c01d4bf03(arg0, arg1) {
    arg0.depthCompare = __wbindgen_enum_GpuCompareFunction[arg1];
};

export function __wbg_set_depth_fail_op_765de27464903fd0(arg0, arg1) {
    arg0.depthFailOp = __wbindgen_enum_GpuStencilOperation[arg1];
};

export function __wbg_set_depth_load_op_33c128108a7dc8f1(arg0, arg1) {
    arg0.depthLoadOp = __wbindgen_enum_GpuLoadOp[arg1];
};

export function __wbg_set_depth_or_array_layers_58d45a4c8cd4f655(arg0, arg1) {
    arg0.depthOrArrayLayers = arg1 >>> 0;
};

export function __wbg_set_depth_read_only_60990818c939df42(arg0, arg1) {
    arg0.depthReadOnly = arg1 !== 0;
};

export function __wbg_set_depth_read_only_fae59572dd12c1c8(arg0, arg1) {
    arg0.depthReadOnly = arg1 !== 0;
};

export function __wbg_set_depth_stencil_2e141a5dfe91878d(arg0, arg1) {
    arg0.depthStencil = arg1;
};

export function __wbg_set_depth_stencil_attachment_47273ec480dd9bb3(arg0, arg1) {
    arg0.depthStencilAttachment = arg1;
};

export function __wbg_set_depth_stencil_format_c9a577086cb44854(arg0, arg1) {
    arg0.depthStencilFormat = __wbindgen_enum_GpuTextureFormat[arg1];
};

export function __wbg_set_depth_store_op_9cf32660e51edb87(arg0, arg1) {
    arg0.depthStoreOp = __wbindgen_enum_GpuStoreOp[arg1];
};

export function __wbg_set_depth_write_enabled_2757b4106a089684(arg0, arg1) {
    arg0.depthWriteEnabled = arg1 !== 0;
};

export function __wbg_set_device_c2cb3231e445ef7c(arg0, arg1) {
    arg0.device = arg1;
};

export function __wbg_set_dimension_0bc5536bd1965aea(arg0, arg1) {
    arg0.dimension = __wbindgen_enum_GpuTextureDimension[arg1];
};

export function __wbg_set_dimension_c7429fee9721a104(arg0, arg1) {
    arg0.dimension = __wbindgen_enum_GpuTextureViewDimension[arg1];
};

export function __wbg_set_dst_factor_976f0a83fd6ab733(arg0, arg1) {
    arg0.dstFactor = __wbindgen_enum_GpuBlendFactor[arg1];
};

export function __wbg_set_end_of_pass_write_index_3cc5a7a3f6819a03(arg0, arg1) {
    arg0.endOfPassWriteIndex = arg1 >>> 0;
};

export function __wbg_set_end_of_pass_write_index_f82ebc8ed8ebaa34(arg0, arg1) {
    arg0.endOfPassWriteIndex = arg1 >>> 0;
};

export function __wbg_set_entries_01031c155d815ef1(arg0, arg1) {
    arg0.entries = arg1;
};

export function __wbg_set_entries_8f49811ca79d7dbf(arg0, arg1) {
    arg0.entries = arg1;
};

export function __wbg_set_entry_point_1da27599bf796782(arg0, arg1, arg2) {
    arg0.entryPoint = getStringFromWasm0(arg1, arg2);
};

export function __wbg_set_entry_point_670e208336b80723(arg0, arg1, arg2) {
    arg0.entryPoint = getStringFromWasm0(arg1, arg2);
};

export function __wbg_set_entry_point_7e39bf2abe77ebae(arg0, arg1, arg2) {
    arg0.entryPoint = getStringFromWasm0(arg1, arg2);
};

export function __wbg_set_external_texture_66700d1d2537a6de(arg0, arg1) {
    arg0.externalTexture = arg1;
};

export function __wbg_set_fail_op_9de9bf69ac6682e3(arg0, arg1) {
    arg0.failOp = __wbindgen_enum_GpuStencilOperation[arg1];
};

export function __wbg_set_flip_y_8e10258813c55af9(arg0, arg1) {
    arg0.flipY = arg1 !== 0;
};

export function __wbg_set_format_10a5222e02236027(arg0, arg1) {
    arg0.format = __wbindgen_enum_GpuTextureFormat[arg1];
};

export function __wbg_set_format_37627c6070d0ecfc(arg0, arg1) {
    arg0.format = __wbindgen_enum_GpuTextureFormat[arg1];
};

export function __wbg_set_format_3c7d4bce3fb94de5(arg0, arg1) {
    arg0.format = __wbindgen_enum_GpuTextureFormat[arg1];
};

export function __wbg_set_format_47fd2845afca8e1a(arg0, arg1) {
    arg0.format = __wbindgen_enum_GpuTextureFormat[arg1];
};

export function __wbg_set_format_72e1ce883fb57e05(arg0, arg1) {
    arg0.format = __wbindgen_enum_GpuTextureFormat[arg1];
};

export function __wbg_set_format_877a89e3431cb656(arg0, arg1) {
    arg0.format = __wbindgen_enum_GpuVertexFormat[arg1];
};

export function __wbg_set_format_ee418ce830040f4d(arg0, arg1) {
    arg0.format = __wbindgen_enum_GpuTextureFormat[arg1];
};

export function __wbg_set_fragment_616c1d1c0db9abd4(arg0, arg1) {
    arg0.fragment = arg1;
};

export function __wbg_set_front_face_a1a0e940bd9fa3d0(arg0, arg1) {
    arg0.frontFace = __wbindgen_enum_GpuFrontFace[arg1];
};

export function __wbg_set_g_9ab482dfe9422850(arg0, arg1) {
    arg0.g = arg1;
};

export function __wbg_set_has_dynamic_offset_21302a736944b6d9(arg0, arg1) {
    arg0.hasDynamicOffset = arg1 !== 0;
};

export function __wbg_set_height_6f8f8ef4cb40e496(arg0, arg1) {
    arg0.height = arg1 >>> 0;
};

export function __wbg_set_height_afe09c24165867f7(arg0, arg1) {
    arg0.height = arg1 >>> 0;
};

export function __wbg_set_height_cd4d12f9029588ee(arg0, arg1) {
    arg0.height = arg1 >>> 0;
};

export function __wbg_set_label_0b21604c6a585153(arg0, arg1, arg2) {
    arg0.label = getStringFromWasm0(arg1, arg2);
};

export function __wbg_set_label_1b7e4bc9d67c38b4(arg0, arg1, arg2) {
    arg0.label = getStringFromWasm0(arg1, arg2);
};

export function __wbg_set_label_2e55e1407bac5ba2(arg0, arg1, arg2) {
    arg0.label = getStringFromWasm0(arg1, arg2);
};

export function __wbg_set_label_407c8b09134f4f1d(arg0, arg1, arg2) {
    arg0.label = getStringFromWasm0(arg1, arg2);
};

export function __wbg_set_label_5dc53fac7117f697(arg0, arg1, arg2) {
    arg0.label = getStringFromWasm0(arg1, arg2);
};

export function __wbg_set_label_8e88157a8e30ddcd(arg0, arg1, arg2) {
    arg0.label = getStringFromWasm0(arg1, arg2);
};

export function __wbg_set_label_8edbc05494bffe0e(arg0, arg1, arg2) {
    arg0.label = getStringFromWasm0(arg1, arg2);
};

export function __wbg_set_label_a56a46194be79e8d(arg0, arg1, arg2) {
    arg0.label = getStringFromWasm0(arg1, arg2);
};

export function __wbg_set_label_a6c76bf653812d73(arg0, arg1, arg2) {
    arg0.label = getStringFromWasm0(arg1, arg2);
};

export function __wbg_set_label_ae972d3c351c79ec(arg0, arg1, arg2) {
    arg0.label = getStringFromWasm0(arg1, arg2);
};

export function __wbg_set_label_b1b0d28716686810(arg0, arg1, arg2) {
    arg0.label = getStringFromWasm0(arg1, arg2);
};

export function __wbg_set_label_cabc4eccde1e89fd(arg0, arg1, arg2) {
    arg0.label = getStringFromWasm0(arg1, arg2);
};

export function __wbg_set_label_cf1bc810a3bd9a59(arg0, arg1, arg2) {
    arg0.label = getStringFromWasm0(arg1, arg2);
};

export function __wbg_set_label_d90e07589bdb8f1a(arg0, arg1, arg2) {
    arg0.label = getStringFromWasm0(arg1, arg2);
};

export function __wbg_set_label_e69d774bf38947d2(arg0, arg1, arg2) {
    arg0.label = getStringFromWasm0(arg1, arg2);
};

export function __wbg_set_label_f401ffe5fc8acb94(arg0, arg1, arg2) {
    arg0.label = getStringFromWasm0(arg1, arg2);
};

export function __wbg_set_label_ff7c2cb9af49bf08(arg0, arg1, arg2) {
    arg0.label = getStringFromWasm0(arg1, arg2);
};

export function __wbg_set_layout_3a36319a5990c8b7(arg0, arg1) {
    arg0.layout = arg1;
};

export function __wbg_set_layout_89fac8ffd04a0d55(arg0, arg1) {
    arg0.layout = arg1;
};

export function __wbg_set_layout_ac044d38ca30f520(arg0, arg1) {
    arg0.layout = arg1;
};

export function __wbg_set_load_op_d48e31970a7bdf9b(arg0, arg1) {
    arg0.loadOp = __wbindgen_enum_GpuLoadOp[arg1];
};

export function __wbg_set_lod_max_clamp_150813b458d7989c(arg0, arg1) {
    arg0.lodMaxClamp = arg1;
};

export function __wbg_set_lod_min_clamp_444adbc1645f8521(arg0, arg1) {
    arg0.lodMinClamp = arg1;
};

export function __wbg_set_mag_filter_4ce311d0e097cca4(arg0, arg1) {
    arg0.magFilter = __wbindgen_enum_GpuFilterMode[arg1];
};

export function __wbg_set_mapped_at_creation_34e7f793131eefbb(arg0, arg1) {
    arg0.mappedAtCreation = arg1 !== 0;
};

export function __wbg_set_mask_a51cdf9e56393e94(arg0, arg1) {
    arg0.mask = arg1 >>> 0;
};

export function __wbg_set_max_anisotropy_5be6e383b6e6632b(arg0, arg1) {
    arg0.maxAnisotropy = arg1;
};

export function __wbg_set_min_binding_size_f9a65ac1a20ab955(arg0, arg1) {
    arg0.minBindingSize = arg1;
};

export function __wbg_set_min_filter_87ee94d6dcfdc3d8(arg0, arg1) {
    arg0.minFilter = __wbindgen_enum_GpuFilterMode[arg1];
};

export function __wbg_set_mip_level_2d7e962e91fd1c33(arg0, arg1) {
    arg0.mipLevel = arg1 >>> 0;
};

export function __wbg_set_mip_level_82be44e699a9cabf(arg0, arg1) {
    arg0.mipLevel = arg1 >>> 0;
};

export function __wbg_set_mip_level_count_32bbfdc1aebc8dd3(arg0, arg1) {
    arg0.mipLevelCount = arg1 >>> 0;
};

export function __wbg_set_mip_level_count_79f47bf6140098e5(arg0, arg1) {
    arg0.mipLevelCount = arg1 >>> 0;
};

export function __wbg_set_mipmap_filter_1739c7c215847dc1(arg0, arg1) {
    arg0.mipmapFilter = __wbindgen_enum_GpuMipmapFilterMode[arg1];
};

export function __wbg_set_module_74f3d1c47da25794(arg0, arg1) {
    arg0.module = arg1;
};

export function __wbg_set_module_8ff6ea5431317fde(arg0, arg1) {
    arg0.module = arg1;
};

export function __wbg_set_module_dae95bb56c7d6ee9(arg0, arg1) {
    arg0.module = arg1;
};

export function __wbg_set_multisample_156e854358e208ff(arg0, arg1) {
    arg0.multisample = arg1;
};

export function __wbg_set_multisampled_775f1e38d554a0f4(arg0, arg1) {
    arg0.multisampled = arg1 !== 0;
};

export function __wbg_set_offset_25f624abc0979ae4(arg0, arg1) {
    arg0.offset = arg1;
};

export function __wbg_set_offset_9cf47ca05ec82222(arg0, arg1) {
    arg0.offset = arg1;
};

export function __wbg_set_offset_9ed8011d53037f93(arg0, arg1) {
    arg0.offset = arg1;
};

export function __wbg_set_offset_d27243aad0b0b017(arg0, arg1) {
    arg0.offset = arg1;
};

export function __wbg_set_onmessage_f0d5bf805190d1d8(arg0, arg1) {
    arg0.onmessage = arg1;
};

export function __wbg_set_onuncapturederror_5abf5ded0c5c6c5f(arg0, arg1) {
    arg0.onuncapturederror = arg1;
};

export function __wbg_set_operation_2ad26b5d94a70e63(arg0, arg1) {
    arg0.operation = __wbindgen_enum_GpuBlendOperation[arg1];
};

export function __wbg_set_origin_0b50b7c9d0cd0d2b(arg0, arg1) {
    arg0.origin = arg1;
};

export function __wbg_set_origin_142f4ec35ba3f8da(arg0, arg1) {
    arg0.origin = arg1;
};

export function __wbg_set_origin_39cb32dbeeb0475a(arg0, arg1) {
    arg0.origin = arg1;
};

export function __wbg_set_pass_op_25209e5db7ec5d4b(arg0, arg1) {
    arg0.passOp = __wbindgen_enum_GpuStencilOperation[arg1];
};

export function __wbg_set_power_preference_2f983dce6d983584(arg0, arg1) {
    arg0.powerPreference = __wbindgen_enum_GpuPowerPreference[arg1];
};

export function __wbg_set_premultiplied_alpha_16b28d8f8575df1b(arg0, arg1) {
    arg0.premultipliedAlpha = arg1 !== 0;
};

export function __wbg_set_primitive_cc91060b2752c577(arg0, arg1) {
    arg0.primitive = arg1;
};

export function __wbg_set_query_set_57ee4e9bc06075da(arg0, arg1) {
    arg0.querySet = arg1;
};

export function __wbg_set_query_set_e258abc9e7072a65(arg0, arg1) {
    arg0.querySet = arg1;
};

export function __wbg_set_r_4943e4c720ff77ca(arg0, arg1) {
    arg0.r = arg1;
};

export function __wbg_set_required_features_52447a9e50ed9b36(arg0, arg1) {
    arg0.requiredFeatures = arg1;
};

export function __wbg_set_resolve_target_28603a69bca08e48(arg0, arg1) {
    arg0.resolveTarget = arg1;
};

export function __wbg_set_resource_0b72a17db4105dcc(arg0, arg1) {
    arg0.resource = arg1;
};

export function __wbg_set_rows_per_image_2388f2cfec4ea946(arg0, arg1) {
    arg0.rowsPerImage = arg1 >>> 0;
};

export function __wbg_set_rows_per_image_d6b2e6d0385b8e27(arg0, arg1) {
    arg0.rowsPerImage = arg1 >>> 0;
};

export function __wbg_set_sample_count_1cd165278e1081cb(arg0, arg1) {
    arg0.sampleCount = arg1 >>> 0;
};

export function __wbg_set_sample_count_8b3966e653c36415(arg0, arg1) {
    arg0.sampleCount = arg1 >>> 0;
};

export function __wbg_set_sample_type_5656761d1d13c084(arg0, arg1) {
    arg0.sampleType = __wbindgen_enum_GpuTextureSampleType[arg1];
};

export function __wbg_set_sampler_9559ad3dd242f711(arg0, arg1) {
    arg0.sampler = arg1;
};

export function __wbg_set_shader_location_2ee098966925fd00(arg0, arg1) {
    arg0.shaderLocation = arg1 >>> 0;
};

export function __wbg_set_size_a43ef8b3ef024e2c(arg0, arg1) {
    arg0.size = arg1;
};

export function __wbg_set_size_d3baf773adcc6357(arg0, arg1) {
    arg0.size = arg1;
};

export function __wbg_set_size_fadeb2bddc7e6f67(arg0, arg1) {
    arg0.size = arg1;
};

export function __wbg_set_source_d446ffccec7cce9a(arg0, arg1) {
    arg0.source = arg1;
};

export function __wbg_set_src_factor_ebc4adbcb746fedc(arg0, arg1) {
    arg0.srcFactor = __wbindgen_enum_GpuBlendFactor[arg1];
};

export function __wbg_set_stencil_back_51d5377faff8840b(arg0, arg1) {
    arg0.stencilBack = arg1;
};

export function __wbg_set_stencil_clear_value_21847cbc9881e39b(arg0, arg1) {
    arg0.stencilClearValue = arg1 >>> 0;
};

export function __wbg_set_stencil_front_115e8b375153cc55(arg0, arg1) {
    arg0.stencilFront = arg1;
};

export function __wbg_set_stencil_load_op_3531e7e23b9c735e(arg0, arg1) {
    arg0.stencilLoadOp = __wbindgen_enum_GpuLoadOp[arg1];
};

export function __wbg_set_stencil_read_mask_6022bedf9e54ec0d(arg0, arg1) {
    arg0.stencilReadMask = arg1 >>> 0;
};

export function __wbg_set_stencil_read_only_02efae715d872f3e(arg0, arg1) {
    arg0.stencilReadOnly = arg1 !== 0;
};

export function __wbg_set_stencil_read_only_beb27fbf4ca9b6e4(arg0, arg1) {
    arg0.stencilReadOnly = arg1 !== 0;
};

export function __wbg_set_stencil_store_op_7b3259ed6b9d76ca(arg0, arg1) {
    arg0.stencilStoreOp = __wbindgen_enum_GpuStoreOp[arg1];
};

export function __wbg_set_stencil_write_mask_294d575eb0e2fd6f(arg0, arg1) {
    arg0.stencilWriteMask = arg1 >>> 0;
};

export function __wbg_set_step_mode_5b6d687e55df5dd0(arg0, arg1) {
    arg0.stepMode = __wbindgen_enum_GpuVertexStepMode[arg1];
};

export function __wbg_set_storage_texture_b2963724a23aca9b(arg0, arg1) {
    arg0.storageTexture = arg1;
};

export function __wbg_set_store_op_e1b7633c5612534a(arg0, arg1) {
    arg0.storeOp = __wbindgen_enum_GpuStoreOp[arg1];
};

export function __wbg_set_strip_index_format_6d0c95e2646c52d1(arg0, arg1) {
    arg0.stripIndexFormat = __wbindgen_enum_GpuIndexFormat[arg1];
};

export function __wbg_set_targets_9f867a93d09515a9(arg0, arg1) {
    arg0.targets = arg1;
};

export function __wbg_set_texture_08516f643ed9f7ef(arg0, arg1) {
    arg0.texture = arg1;
};

export function __wbg_set_texture_5f5d866a27cda2f3(arg0, arg1) {
    arg0.texture = arg1;
};

export function __wbg_set_texture_fbeffa5f2e57db49(arg0, arg1) {
    arg0.texture = arg1;
};

export function __wbg_set_timestamp_writes_54b499e0902d7146(arg0, arg1) {
    arg0.timestampWrites = arg1;
};

export function __wbg_set_timestamp_writes_94da76b5f3fee792(arg0, arg1) {
    arg0.timestampWrites = arg1;
};

export function __wbg_set_topology_0ef9190b0c51fc78(arg0, arg1) {
    arg0.topology = __wbindgen_enum_GpuPrimitiveTopology[arg1];
};

export function __wbg_set_type_3b563491184d1c74(arg0, arg1) {
    arg0.type = __wbindgen_enum_GpuQueryType[arg1];
};

export function __wbg_set_type_657cd6d704dbc037(arg0, arg1) {
    arg0.type = __wbindgen_enum_GpuBufferBindingType[arg1];
};

export function __wbg_set_type_7ce650670a34c68f(arg0, arg1, arg2) {
    arg0.type = getStringFromWasm0(arg1, arg2);
};

export function __wbg_set_type_c9565dd4ebe21c60(arg0, arg1) {
    arg0.type = __wbindgen_enum_GpuSamplerBindingType[arg1];
};

export function __wbg_set_unclipped_depth_936bc9a32a318b94(arg0, arg1) {
    arg0.unclippedDepth = arg1 !== 0;
};

export function __wbg_set_usage_500c45ebe8b0bbf2(arg0, arg1) {
    arg0.usage = arg1 >>> 0;
};

export function __wbg_set_usage_9c6ccd6bcc15f735(arg0, arg1) {
    arg0.usage = arg1 >>> 0;
};

export function __wbg_set_usage_b84e5d16af27594a(arg0, arg1) {
    arg0.usage = arg1 >>> 0;
};

export function __wbg_set_usage_e2790ec1205a5e27(arg0, arg1) {
    arg0.usage = arg1 >>> 0;
};

export function __wbg_set_vertex_9c9752039687305f(arg0, arg1) {
    arg0.vertex = arg1;
};

export function __wbg_set_view_5aa6ed9f881b63f2(arg0, arg1) {
    arg0.view = arg1;
};

export function __wbg_set_view_820375e4a740874f(arg0, arg1) {
    arg0.view = arg1;
};

export function __wbg_set_view_dimension_6ba3ac8e6bedbcb4(arg0, arg1) {
    arg0.viewDimension = __wbindgen_enum_GpuTextureViewDimension[arg1];
};

export function __wbg_set_view_dimension_95e6461d131f7086(arg0, arg1) {
    arg0.viewDimension = __wbindgen_enum_GpuTextureViewDimension[arg1];
};

export function __wbg_set_view_formats_6533614c7017475e(arg0, arg1) {
    arg0.viewFormats = arg1;
};

export function __wbg_set_view_formats_ff46db459c40096d(arg0, arg1) {
    arg0.viewFormats = arg1;
};

export function __wbg_set_visibility_deca18896989c982(arg0, arg1) {
    arg0.visibility = arg1 >>> 0;
};

export function __wbg_set_width_07eabc802de7b030(arg0, arg1) {
    arg0.width = arg1 >>> 0;
};

export function __wbg_set_width_0a22c810f06a5152(arg0, arg1) {
    arg0.width = arg1 >>> 0;
};

export function __wbg_set_width_7ff7a22c6e9f423e(arg0, arg1) {
    arg0.width = arg1 >>> 0;
};

export function __wbg_set_write_mask_122c167c45bb2d8e(arg0, arg1) {
    arg0.writeMask = arg1 >>> 0;
};

export function __wbg_set_x_be1ec46ce6627cfc(arg0, arg1) {
    arg0.x = arg1 >>> 0;
};

export function __wbg_set_x_cc281962ce68ef00(arg0, arg1) {
    arg0.x = arg1 >>> 0;
};

export function __wbg_set_y_71fc9939d0375491(arg0, arg1) {
    arg0.y = arg1 >>> 0;
};

export function __wbg_set_y_7d6f1f0a01ce4000(arg0, arg1) {
    arg0.y = arg1 >>> 0;
};

export function __wbg_set_z_b316da2a41e7822f(arg0, arg1) {
    arg0.z = arg1 >>> 0;
};

export function __wbg_shiftKey_a6df227a917d203b(arg0) {
    const ret = arg0.shiftKey;
    return ret;
};

export function __wbg_shiftKey_d2640abcfa98acec(arg0) {
    const ret = arg0.shiftKey;
    return ret;
};

export function __wbg_signal_3c14fbdc89694b39(arg0) {
    const ret = arg0.signal;
    return ret;
};

export function __wbg_size_beea1890c315fb17(arg0) {
    const ret = arg0.size;
    return ret;
};

export function __wbg_stack_0ed75d68575b0f3c(arg0, arg1) {
    const ret = arg1.stack;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};

export function __wbg_start_dd05b3be5674e9f3(arg0) {
    arg0.start();
};

export function __wbg_static_accessor_GLOBAL_769e6b65d6557335() {
    const ret = typeof global === 'undefined' ? null : global;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};

export function __wbg_static_accessor_GLOBAL_THIS_60cf02db4de8e1c1() {
    const ret = typeof globalThis === 'undefined' ? null : globalThis;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};

export function __wbg_static_accessor_SELF_08f5a74c69739274() {
    const ret = typeof self === 'undefined' ? null : self;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};

export function __wbg_static_accessor_WINDOW_a8924b26aa92d024() {
    const ret = typeof window === 'undefined' ? null : window;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};

export function __wbg_style_521a717da50e53c6(arg0) {
    const ret = arg0.style;
    return ret;
};

export function __wbg_submit_3ecd36be9abeba75(arg0, arg1) {
    arg0.submit(arg1);
};

export function __wbg_then_429f7caf1026411d(arg0, arg1, arg2) {
    const ret = arg0.then(arg1, arg2);
    return ret;
};

export function __wbg_then_4f95312d68691235(arg0, arg1) {
    const ret = arg0.then(arg1);
    return ret;
};

export function __wbg_unmap_2903d5b193373f12(arg0) {
    arg0.unmap();
};

export function __wbg_unobserve_0d3c5074b9205239(arg0, arg1) {
    arg0.unobserve(arg1);
};

export function __wbg_usage_7b00ab14a235fa77(arg0) {
    const ret = arg0.usage;
    return ret;
};

export function __wbg_userAgentData_f7b0e61c05c54315(arg0) {
    const ret = arg0.userAgentData;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};

export function __wbg_userAgent_e18bc0cc9ad38ec1() { return handleError(function (arg0, arg1) {
    const ret = arg1.userAgent;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
}, arguments) };

export function __wbg_valueOf_663ea9f1ad0d6eda(arg0) {
    const ret = arg0.valueOf();
    return ret;
};

export function __wbg_value_57b7b035e117f7ee(arg0) {
    const ret = arg0.value;
    return ret;
};

export function __wbg_visibilityState_2f27cbaac764b521(arg0) {
    const ret = arg0.visibilityState;
    return (__wbindgen_enum_VisibilityState.indexOf(ret) + 1 || 3) - 1;
};

export function __wbg_warn_6e567d0d926ff881(arg0) {
    console.warn(arg0);
};

export function __wbg_webkitExitFullscreen_85426cef5e755dfa(arg0) {
    arg0.webkitExitFullscreen();
};

export function __wbg_webkitFullscreenElement_a9ca38b7214d1567(arg0) {
    const ret = arg0.webkitFullscreenElement;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};

export function __wbg_webkitRequestFullscreen_23664c63833ff0e5(arg0) {
    arg0.webkitRequestFullscreen();
};

export function __wbg_wgslLanguageFeatures_573953bc7ddeb467(arg0) {
    const ret = arg0.wgslLanguageFeatures;
    return ret;
};

export function __wbg_width_30d712cfe70e4fae(arg0) {
    const ret = arg0.width;
    return ret;
};

export function __wbg_writeBuffer_1897edb8e6677e9a() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5) {
    arg0.writeBuffer(arg1, arg2, arg3, arg4, arg5);
}, arguments) };

export function __wbg_writeTexture_e6008247063eadbf() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
    arg0.writeTexture(arg1, arg2, arg3, arg4);
}, arguments) };

export function __wbindgen_cast_2241b6af4c4b2941(arg0, arg1) {
    // Cast intrinsic for `Ref(String) -> Externref`.
    const ret = getStringFromWasm0(arg0, arg1);
    return ret;
};

export function __wbindgen_cast_4201c4d39e34e832(arg0, arg1) {
    // Cast intrinsic for `Closure(Closure { dtor_idx: 382, function: Function { arguments: [NamedExternref("WheelEvent")], shim_idx: 383, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
    const ret = makeMutClosure(arg0, arg1, wasm.wasm_bindgen__closure__destroy__h81b0014b7004c6d6, wasm_bindgen__convert__closures_____invoke__hb5c2a4e092461377);
    return ret;
};

export function __wbindgen_cast_4de909aab903ee4c(arg0, arg1) {
    // Cast intrinsic for `Closure(Closure { dtor_idx: 380, function: Function { arguments: [NamedExternref("KeyboardEvent")], shim_idx: 381, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
    const ret = makeMutClosure(arg0, arg1, wasm.wasm_bindgen__closure__destroy__h5ba5449a24dc04df, wasm_bindgen__convert__closures_____invoke__hfce13e71b68b9f78);
    return ret;
};

export function __wbindgen_cast_4f64743d6f6010ca(arg0, arg1) {
    // Cast intrinsic for `Closure(Closure { dtor_idx: 386, function: Function { arguments: [NamedExternref("PageTransitionEvent")], shim_idx: 387, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
    const ret = makeMutClosure(arg0, arg1, wasm.wasm_bindgen__closure__destroy__h7530440220bcb8bf, wasm_bindgen__convert__closures_____invoke__h5997bd2378143005);
    return ret;
};

export function __wbindgen_cast_636e655be9a36a53(arg0, arg1) {
    // Cast intrinsic for `Closure(Closure { dtor_idx: 388, function: Function { arguments: [], shim_idx: 389, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
    const ret = makeMutClosure(arg0, arg1, wasm.wasm_bindgen__closure__destroy__h6c14e0bbaa33f71e, wasm_bindgen__convert__closures_____invoke__hdb80f1ed7129c4f1);
    return ret;
};

export function __wbindgen_cast_67d14185bb436b4e(arg0, arg1) {
    // Cast intrinsic for `Closure(Closure { dtor_idx: 726, function: Function { arguments: [Externref], shim_idx: 727, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
    const ret = makeMutClosure(arg0, arg1, wasm.wasm_bindgen__closure__destroy__h3786c7b017e19f4d, wasm_bindgen__convert__closures_____invoke__h5bdae1983e88d88e);
    return ret;
};

export function __wbindgen_cast_708092e73c1005f4(arg0, arg1) {
    // Cast intrinsic for `Closure(Closure { dtor_idx: 376, function: Function { arguments: [NamedExternref("Array<any>"), NamedExternref("ResizeObserver")], shim_idx: 377, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
    const ret = makeMutClosure(arg0, arg1, wasm.wasm_bindgen__closure__destroy__h4bc839e0a199cb13, wasm_bindgen__convert__closures_____invoke__h7b85597c6c141fa5);
    return ret;
};

export function __wbindgen_cast_7117a4d405078350(arg0, arg1) {
    // Cast intrinsic for `Closure(Closure { dtor_idx: 390, function: Function { arguments: [NamedExternref("Event")], shim_idx: 391, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
    const ret = makeMutClosure(arg0, arg1, wasm.wasm_bindgen__closure__destroy__hba469e99f58e502a, wasm_bindgen__convert__closures_____invoke__h9233806efcfd5643);
    return ret;
};

export function __wbindgen_cast_79bc6597b450dee7(arg0, arg1) {
    // Cast intrinsic for `Closure(Closure { dtor_idx: 374, function: Function { arguments: [NamedExternref("FocusEvent")], shim_idx: 375, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
    const ret = makeMutClosure(arg0, arg1, wasm.wasm_bindgen__closure__destroy__ha031fb354715f687, wasm_bindgen__convert__closures_____invoke__hd7f2cee65b407601);
    return ret;
};

export function __wbindgen_cast_cb9088102bce6b30(arg0, arg1) {
    // Cast intrinsic for `Ref(Slice(U8)) -> NamedExternref("Uint8Array")`.
    const ret = getArrayU8FromWasm0(arg0, arg1);
    return ret;
};

export function __wbindgen_cast_d22509dace399b22(arg0, arg1) {
    // Cast intrinsic for `Closure(Closure { dtor_idx: 564, function: Function { arguments: [NamedExternref("GPUUncapturedErrorEvent")], shim_idx: 569, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
    const ret = makeMutClosure(arg0, arg1, wasm.wasm_bindgen__closure__destroy__hb99f7865e774358d, wasm_bindgen__convert__closures_____invoke__h952098ab27c9525c);
    return ret;
};

export function __wbindgen_cast_d6cd19b81560fd6e(arg0) {
    // Cast intrinsic for `F64 -> Externref`.
    const ret = arg0;
    return ret;
};

export function __wbindgen_cast_ecc2d4a8e05d3346(arg0, arg1) {
    // Cast intrinsic for `Closure(Closure { dtor_idx: 384, function: Function { arguments: [NamedExternref("PointerEvent")], shim_idx: 385, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
    const ret = makeMutClosure(arg0, arg1, wasm.wasm_bindgen__closure__destroy__h527b93f4e1d2d1da, wasm_bindgen__convert__closures_____invoke__hda95989a6850e8ae);
    return ret;
};

export function __wbindgen_cast_fe6ce8873d373bfb(arg0, arg1) {
    // Cast intrinsic for `Closure(Closure { dtor_idx: 378, function: Function { arguments: [NamedExternref("Array<any>")], shim_idx: 379, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
    const ret = makeMutClosure(arg0, arg1, wasm.wasm_bindgen__closure__destroy__h8b8db0507dfe8f07, wasm_bindgen__convert__closures_____invoke__h72e1e188793d6dc6);
    return ret;
};

export function __wbindgen_init_externref_table() {
    const table = wasm.__wbindgen_externrefs;
    const offset = table.grow(4);
    table.set(0, undefined);
    table.set(offset + 0, undefined);
    table.set(offset + 1, null);
    table.set(offset + 2, true);
    table.set(offset + 3, false);
};
