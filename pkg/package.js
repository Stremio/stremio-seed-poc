(function() {
    const __exports = {};
    let wasm;

    let cachedTextDecoder = new TextDecoder('utf-8');

    let cachegetUint8Memory = null;
    function getUint8Memory() {
        if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== wasm.memory.buffer) {
            cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
        }
        return cachegetUint8Memory;
    }

    function getStringFromWasm(ptr, len) {
        return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
    }

    __exports.__wbg_error_4bb6c2a97407129a = function(arg0, arg1) {
        let varg0 = getStringFromWasm(arg0, arg1);

        varg0 = varg0.slice();
        wasm.__wbindgen_free(arg0, arg1 * 1);

        console.error(varg0);
    };

    const heap = new Array(32);

    heap.fill(undefined);

    heap.push(undefined, null, true, false);

    let heap_next = heap.length;

    function addHeapObject(obj) {
        if (heap_next === heap.length) heap.push(heap.length + 1);
        const idx = heap_next;
        heap_next = heap[idx];

        heap[idx] = obj;
        return idx;
    }

    __exports.__wbg_new_59cb74e423758ede = function() {
        return addHeapObject(new Error());
    };

function getObject(idx) { return heap[idx]; }

let WASM_VECTOR_LEN = 0;

let cachedTextEncoder = new TextEncoder('utf-8');

let passStringToWasm;
if (typeof cachedTextEncoder.encodeInto === 'function') {
    passStringToWasm = function(arg) {


        let size = arg.length;
        let ptr = wasm.__wbindgen_malloc(size);
        let offset = 0;
        {
            const mem = getUint8Memory();
            for (; offset < arg.length; offset++) {
                const code = arg.charCodeAt(offset);
                if (code > 0x7F) break;
                mem[ptr + offset] = code;
            }
        }

        if (offset !== arg.length) {
            arg = arg.slice(offset);
            ptr = wasm.__wbindgen_realloc(ptr, size, size = offset + arg.length * 3);
            const view = getUint8Memory().subarray(ptr + offset, ptr + size);
            const ret = cachedTextEncoder.encodeInto(arg, view);

            offset += ret.written;
        }
        WASM_VECTOR_LEN = offset;
        return ptr;
    };
} else {
    passStringToWasm = function(arg) {


        let size = arg.length;
        let ptr = wasm.__wbindgen_malloc(size);
        let offset = 0;
        {
            const mem = getUint8Memory();
            for (; offset < arg.length; offset++) {
                const code = arg.charCodeAt(offset);
                if (code > 0x7F) break;
                mem[ptr + offset] = code;
            }
        }

        if (offset !== arg.length) {
            const buf = cachedTextEncoder.encode(arg.slice(offset));
            ptr = wasm.__wbindgen_realloc(ptr, size, size = offset + buf.length);
            getUint8Memory().set(buf, ptr + offset);
            offset += buf.length;
        }
        WASM_VECTOR_LEN = offset;
        return ptr;
    };
}

let cachegetUint32Memory = null;
function getUint32Memory() {
    if (cachegetUint32Memory === null || cachegetUint32Memory.buffer !== wasm.memory.buffer) {
        cachegetUint32Memory = new Uint32Array(wasm.memory.buffer);
    }
    return cachegetUint32Memory;
}

__exports.__wbg_stack_558ba5917b466edd = function(ret, arg0) {

    const retptr = passStringToWasm(getObject(arg0).stack);
    const retlen = WASM_VECTOR_LEN;
    const mem = getUint32Memory();
    mem[ret / 4] = retptr;
    mem[ret / 4 + 1] = retlen;

};

function handleError(exnptr, e) {
    const view = getUint32Memory();
    view[exnptr / 4] = 1;
    view[exnptr / 4 + 1] = addHeapObject(e);
}

__exports.__widl_f_create_element_Document = function(arg0, arg1, arg2, exnptr) {
    let varg1 = getStringFromWasm(arg1, arg2);
    try {
        return addHeapObject(getObject(arg0).createElement(varg1));
    } catch (e) {
        handleError(exnptr, e);
    }
};

__exports.__widl_f_create_element_ns_Document = function(arg0, arg1, arg2, arg3, arg4, exnptr) {
    let varg1 = arg1 == 0 ? undefined : getStringFromWasm(arg1, arg2);
    let varg3 = getStringFromWasm(arg3, arg4);
    try {
        return addHeapObject(getObject(arg0).createElementNS(varg1, varg3));
    } catch (e) {
        handleError(exnptr, e);
    }
};

__exports.__widl_f_create_text_node_Document = function(arg0, arg1, arg2) {
    let varg1 = getStringFromWasm(arg1, arg2);
    return addHeapObject(getObject(arg0).createTextNode(varg1));
};

function isLikeNone(x) {
    return x === undefined || x === null;
}

__exports.__widl_f_get_element_by_id_Document = function(arg0, arg1, arg2) {
    let varg1 = getStringFromWasm(arg1, arg2);

    const val = getObject(arg0).getElementById(varg1);
    return isLikeNone(val) ? 0 : addHeapObject(val);

};

__exports.__widl_instanceof_Element = function(idx) { return getObject(idx) instanceof Element ? 1 : 0; };

__exports.__widl_f_get_attribute_Element = function(ret, arg0, arg1, arg2) {
    let varg1 = getStringFromWasm(arg1, arg2);
    const val = getObject(arg0).getAttribute(varg1);
    const retptr = isLikeNone(val) ? [0, 0] : passStringToWasm(val);
    const retlen = WASM_VECTOR_LEN;
    const mem = getUint32Memory();
    mem[ret / 4] = retptr;
    mem[ret / 4 + 1] = retlen;

};

__exports.__widl_f_remove_attribute_Element = function(arg0, arg1, arg2, exnptr) {
    let varg1 = getStringFromWasm(arg1, arg2);
    try {
        getObject(arg0).removeAttribute(varg1);
    } catch (e) {
        handleError(exnptr, e);
    }
};

__exports.__widl_f_set_attribute_Element = function(arg0, arg1, arg2, arg3, arg4, exnptr) {
    let varg1 = getStringFromWasm(arg1, arg2);
    let varg3 = getStringFromWasm(arg3, arg4);
    try {
        getObject(arg0).setAttribute(varg1, varg3);
    } catch (e) {
        handleError(exnptr, e);
    }
};

__exports.__widl_f_tag_name_Element = function(ret, arg0) {

    const retptr = passStringToWasm(getObject(arg0).tagName);
    const retlen = WASM_VECTOR_LEN;
    const mem = getUint32Memory();
    mem[ret / 4] = retptr;
    mem[ret / 4 + 1] = retlen;

};

__exports.__widl_f_prevent_default_Event = function(arg0) {
    getObject(arg0).preventDefault();
};

__exports.__widl_f_target_Event = function(arg0) {

    const val = getObject(arg0).target;
    return isLikeNone(val) ? 0 : addHeapObject(val);

};

__exports.__widl_f_add_event_listener_with_callback_EventTarget = function(arg0, arg1, arg2, arg3, exnptr) {
    let varg1 = getStringFromWasm(arg1, arg2);
    try {
        getObject(arg0).addEventListener(varg1, getObject(arg3));
    } catch (e) {
        handleError(exnptr, e);
    }
};

__exports.__widl_f_remove_event_listener_with_callback_EventTarget = function(arg0, arg1, arg2, arg3, exnptr) {
    let varg1 = getStringFromWasm(arg1, arg2);
    try {
        getObject(arg0).removeEventListener(varg1, getObject(arg3));
    } catch (e) {
        handleError(exnptr, e);
    }
};

__exports.__widl_instanceof_HTMLButtonElement = function(idx) { return getObject(idx) instanceof HTMLButtonElement ? 1 : 0; };

__exports.__widl_f_set_autofocus_HTMLButtonElement = function(arg0, arg1) {
    getObject(arg0).autofocus = arg1 !== 0;
};

__exports.__widl_instanceof_HTMLInputElement = function(idx) { return getObject(idx) instanceof HTMLInputElement ? 1 : 0; };

__exports.__widl_f_set_autofocus_HTMLInputElement = function(arg0, arg1) {
    getObject(arg0).autofocus = arg1 !== 0;
};

__exports.__widl_f_checked_HTMLInputElement = function(arg0) {
    return getObject(arg0).checked;
};

__exports.__widl_f_set_checked_HTMLInputElement = function(arg0, arg1) {
    getObject(arg0).checked = arg1 !== 0;
};

__exports.__widl_f_type_HTMLInputElement = function(ret, arg0) {

    const retptr = passStringToWasm(getObject(arg0).type);
    const retlen = WASM_VECTOR_LEN;
    const mem = getUint32Memory();
    mem[ret / 4] = retptr;
    mem[ret / 4 + 1] = retlen;

};

__exports.__widl_f_default_value_HTMLInputElement = function(ret, arg0) {

    const retptr = passStringToWasm(getObject(arg0).defaultValue);
    const retlen = WASM_VECTOR_LEN;
    const mem = getUint32Memory();
    mem[ret / 4] = retptr;
    mem[ret / 4 + 1] = retlen;

};

__exports.__widl_f_value_HTMLInputElement = function(ret, arg0) {

    const retptr = passStringToWasm(getObject(arg0).value);
    const retlen = WASM_VECTOR_LEN;
    const mem = getUint32Memory();
    mem[ret / 4] = retptr;
    mem[ret / 4 + 1] = retlen;

};

__exports.__widl_f_set_value_HTMLInputElement = function(arg0, arg1, arg2) {
    let varg1 = getStringFromWasm(arg1, arg2);
    getObject(arg0).value = varg1;
};

__exports.__widl_instanceof_HTMLSelectElement = function(idx) { return getObject(idx) instanceof HTMLSelectElement ? 1 : 0; };

__exports.__widl_f_set_autofocus_HTMLSelectElement = function(arg0, arg1) {
    getObject(arg0).autofocus = arg1 !== 0;
};

__exports.__widl_f_value_HTMLSelectElement = function(ret, arg0) {

    const retptr = passStringToWasm(getObject(arg0).value);
    const retlen = WASM_VECTOR_LEN;
    const mem = getUint32Memory();
    mem[ret / 4] = retptr;
    mem[ret / 4 + 1] = retlen;

};

__exports.__widl_f_set_value_HTMLSelectElement = function(arg0, arg1, arg2) {
    let varg1 = getStringFromWasm(arg1, arg2);
    getObject(arg0).value = varg1;
};

__exports.__widl_instanceof_HTMLTextAreaElement = function(idx) { return getObject(idx) instanceof HTMLTextAreaElement ? 1 : 0; };

__exports.__widl_f_set_autofocus_HTMLTextAreaElement = function(arg0, arg1) {
    getObject(arg0).autofocus = arg1 !== 0;
};

__exports.__widl_f_value_HTMLTextAreaElement = function(ret, arg0) {

    const retptr = passStringToWasm(getObject(arg0).value);
    const retlen = WASM_VECTOR_LEN;
    const mem = getUint32Memory();
    mem[ret / 4] = retptr;
    mem[ret / 4 + 1] = retlen;

};

__exports.__widl_f_set_value_HTMLTextAreaElement = function(arg0, arg1, arg2) {
    let varg1 = getStringFromWasm(arg1, arg2);
    getObject(arg0).value = varg1;
};

__exports.__widl_f_push_state_with_url_History = function(arg0, arg1, arg2, arg3, arg4, arg5, exnptr) {
    let varg2 = getStringFromWasm(arg2, arg3);
    let varg4 = arg4 == 0 ? undefined : getStringFromWasm(arg4, arg5);
    try {
        getObject(arg0).pushState(getObject(arg1), varg2, varg4);
    } catch (e) {
        handleError(exnptr, e);
    }
};

__exports.__widl_f_pathname_Location = function(ret, arg0, exnptr) {
    try {

        const retptr = passStringToWasm(getObject(arg0).pathname);
        const retlen = WASM_VECTOR_LEN;
        const mem = getUint32Memory();
        mem[ret / 4] = retptr;
        mem[ret / 4 + 1] = retlen;

    } catch (e) {
        handleError(exnptr, e);
    }
};

__exports.__widl_f_search_Location = function(ret, arg0, exnptr) {
    try {

        const retptr = passStringToWasm(getObject(arg0).search);
        const retlen = WASM_VECTOR_LEN;
        const mem = getUint32Memory();
        mem[ret / 4] = retptr;
        mem[ret / 4 + 1] = retlen;

    } catch (e) {
        handleError(exnptr, e);
    }
};

__exports.__widl_f_hash_Location = function(ret, arg0, exnptr) {
    try {

        const retptr = passStringToWasm(getObject(arg0).hash);
        const retlen = WASM_VECTOR_LEN;
        const mem = getUint32Memory();
        mem[ret / 4] = retptr;
        mem[ret / 4 + 1] = retlen;

    } catch (e) {
        handleError(exnptr, e);
    }
};

__exports.__widl_f_append_child_Node = function(arg0, arg1, exnptr) {
    try {
        return addHeapObject(getObject(arg0).appendChild(getObject(arg1)));
    } catch (e) {
        handleError(exnptr, e);
    }
};

__exports.__widl_f_insert_before_Node = function(arg0, arg1, arg2, exnptr) {
    try {
        return addHeapObject(getObject(arg0).insertBefore(getObject(arg1), getObject(arg2)));
    } catch (e) {
        handleError(exnptr, e);
    }
};

__exports.__widl_f_remove_child_Node = function(arg0, arg1, exnptr) {
    try {
        return addHeapObject(getObject(arg0).removeChild(getObject(arg1)));
    } catch (e) {
        handleError(exnptr, e);
    }
};

__exports.__widl_f_replace_child_Node = function(arg0, arg1, arg2, exnptr) {
    try {
        return addHeapObject(getObject(arg0).replaceChild(getObject(arg1), getObject(arg2)));
    } catch (e) {
        handleError(exnptr, e);
    }
};

__exports.__widl_f_node_type_Node = function(arg0) {
    return getObject(arg0).nodeType;
};

__exports.__widl_f_first_child_Node = function(arg0) {

    const val = getObject(arg0).firstChild;
    return isLikeNone(val) ? 0 : addHeapObject(val);

};

__exports.__widl_f_next_sibling_Node = function(arg0) {

    const val = getObject(arg0).nextSibling;
    return isLikeNone(val) ? 0 : addHeapObject(val);

};

__exports.__widl_f_set_text_content_Node = function(arg0, arg1, arg2) {
    let varg1 = arg1 == 0 ? undefined : getStringFromWasm(arg1, arg2);
    getObject(arg0).textContent = varg1;
};

__exports.__widl_instanceof_PopStateEvent = function(idx) { return getObject(idx) instanceof PopStateEvent ? 1 : 0; };

__exports.__widl_f_state_PopStateEvent = function(arg0) {
    return addHeapObject(getObject(arg0).state);
};

__exports.__widl_f_new_with_str_and_init_Request = function(arg0, arg1, arg2, exnptr) {
    let varg0 = getStringFromWasm(arg0, arg1);
    try {
        return addHeapObject(new Request(varg0, getObject(arg2)));
    } catch (e) {
        handleError(exnptr, e);
    }
};

__exports.__widl_instanceof_Response = function(idx) { return getObject(idx) instanceof Response ? 1 : 0; };

__exports.__widl_f_json_Response = function(arg0, exnptr) {
    try {
        return addHeapObject(getObject(arg0).json());
    } catch (e) {
        handleError(exnptr, e);
    }
};

__exports.__widl_f_get_item_Storage = function(ret, arg0, arg1, arg2, exnptr) {
    let varg1 = getStringFromWasm(arg1, arg2);
    try {
        const val = getObject(arg0).getItem(varg1);
        const retptr = isLikeNone(val) ? [0, 0] : passStringToWasm(val);
        const retlen = WASM_VECTOR_LEN;
        const mem = getUint32Memory();
        mem[ret / 4] = retptr;
        mem[ret / 4 + 1] = retlen;

    } catch (e) {
        handleError(exnptr, e);
    }
};

__exports.__widl_f_set_item_Storage = function(arg0, arg1, arg2, arg3, arg4, exnptr) {
    let varg1 = getStringFromWasm(arg1, arg2);
    let varg3 = getStringFromWasm(arg3, arg4);
    try {
        getObject(arg0).setItem(varg1, varg3);
    } catch (e) {
        handleError(exnptr, e);
    }
};

__exports.__widl_instanceof_Window = function(idx) { return getObject(idx) instanceof Window ? 1 : 0; };

__exports.__widl_f_document_Window = function(arg0) {

    const val = getObject(arg0).document;
    return isLikeNone(val) ? 0 : addHeapObject(val);

};

__exports.__widl_f_location_Window = function(arg0) {
    return addHeapObject(getObject(arg0).location);
};

__exports.__widl_f_history_Window = function(arg0, exnptr) {
    try {
        return addHeapObject(getObject(arg0).history);
    } catch (e) {
        handleError(exnptr, e);
    }
};

__exports.__widl_f_local_storage_Window = function(arg0, exnptr) {
    try {

        const val = getObject(arg0).localStorage;
        return isLikeNone(val) ? 0 : addHeapObject(val);

    } catch (e) {
        handleError(exnptr, e);
    }
};

__exports.__widl_f_fetch_with_request_Window = function(arg0, arg1) {
    return addHeapObject(getObject(arg0).fetch(getObject(arg1)));
};

__exports.__widl_f_error_1_ = function(arg0) {
    console.error(getObject(arg0));
};

__exports.__widl_f_log_1_ = function(arg0) {
    console.log(getObject(arg0));
};

__exports.__wbg_instanceof_Error_7b8282f5e91987e7 = function(idx) { return getObject(idx) instanceof Error ? 1 : 0; };

__exports.__wbg_toString_23df02ba028af846 = function(arg0) {
    return addHeapObject(getObject(arg0).toString());
};

__exports.__wbg_newnoargs_a172f39151049128 = function(arg0, arg1) {
    let varg0 = getStringFromWasm(arg0, arg1);
    return addHeapObject(new Function(varg0));
};

__exports.__wbg_call_8a9c8b0a32a202ff = function(arg0, arg1, exnptr) {
    try {
        return addHeapObject(getObject(arg0).call(getObject(arg1)));
    } catch (e) {
        handleError(exnptr, e);
    }
};

__exports.__wbg_call_6eed851d946419d8 = function(arg0, arg1, arg2, exnptr) {
    try {
        return addHeapObject(getObject(arg0).call(getObject(arg1), getObject(arg2)));
    } catch (e) {
        handleError(exnptr, e);
    }
};

__exports.__wbg_new_68180085d411e1be = function() {
    return addHeapObject(new Object());
};

__exports.__wbg_new_7884206509213d02 = function(arg0, arg1) {
    let cbarg0 = function(arg0, arg1) {
        let a = this.a;
        this.a = 0;
        try {
            return this.f(a, this.b, addHeapObject(arg0), addHeapObject(arg1));

        } finally {
            this.a = a;

        }

    };
    cbarg0.f = wasm.__wbg_function_table.get(81);
    cbarg0.a = arg0;
    cbarg0.b = arg1;
    try {
        return addHeapObject(new Promise(cbarg0.bind(cbarg0)));
    } finally {
        cbarg0.a = cbarg0.b = 0;

    }
};

__exports.__wbg_resolve_7f1a97b982aed82d = function(arg0) {
    return addHeapObject(Promise.resolve(getObject(arg0)));
};

__exports.__wbg_then_ad10ddb5ba4496f4 = function(arg0, arg1) {
    return addHeapObject(getObject(arg0).then(getObject(arg1)));
};

__exports.__wbg_then_bd535717286c5b54 = function(arg0, arg1, arg2) {
    return addHeapObject(getObject(arg0).then(getObject(arg1), getObject(arg2)));
};

__exports.__wbg_set_8866dbb36cf947cb = function(arg0, arg1, arg2, exnptr) {
    try {
        return Reflect.set(getObject(arg0), getObject(arg1), getObject(arg2));
    } catch (e) {
        handleError(exnptr, e);
    }
};
/**
* @returns {void}
*/
__exports.render = function() {
    return wasm.render();
};

__exports.__wbindgen_string_new = function(p, l) { return addHeapObject(getStringFromWasm(p, l)); };

__exports.__wbindgen_string_get = function(i, len_ptr) {
    let obj = getObject(i);
    if (typeof(obj) !== 'string') return 0;
    const ptr = passStringToWasm(obj);
    getUint32Memory()[len_ptr / 4] = WASM_VECTOR_LEN;
    return ptr;
};

__exports.__wbindgen_debug_string = function(i, len_ptr) {
    const debug_str =
    val => {
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
                debug += debug_str(val[0]);
            }
            for(let i = 1; i < length; i++) {
                debug += ', ' + debug_str(val[i]);
            }
            debug += ']';
            return debug;
        }
        // Test for built-in
        const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
        let className;
        if (builtInMatches.length > 1) {
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
        return `${val.name}: ${val.message}
        ${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}
;
const toString = Object.prototype.toString;
const val = getObject(i);
const debug = debug_str(val);
const ptr = passStringToWasm(debug);
getUint32Memory()[len_ptr / 4] = WASM_VECTOR_LEN;
return ptr;
};

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

__exports.__wbindgen_cb_drop = function(i) {
    const obj = takeObject(i).original;
    if (obj.cnt-- == 1) {
        obj.a = 0;
        return 1;
    }
    return 0;
};

__exports.__wbindgen_cb_forget = dropObject;

__exports.__wbindgen_json_parse = function(ptr, len) { return addHeapObject(JSON.parse(getStringFromWasm(ptr, len))); };

__exports.__wbindgen_json_serialize = function(idx, ptrptr) {
    const ptr = passStringToWasm(JSON.stringify(getObject(idx)));
    getUint32Memory()[ptrptr / 4] = ptr;
    return WASM_VECTOR_LEN;
};

__exports.__wbindgen_throw = function(ptr, len) {
    throw new Error(getStringFromWasm(ptr, len));
};

__exports.__wbindgen_closure_wrapper2573 = function(a, b, _ignored) {
    const f = wasm.__wbg_function_table.get(71);
    const d = wasm.__wbg_function_table.get(72);
    const cb = function(arg0) {
        this.cnt++;
        let a = this.a;
        this.a = 0;
        try {
            return f(a, b, addHeapObject(arg0));

        } finally {
            if (--this.cnt === 0) d(a, b);
            else this.a = a;

        }

    };
    cb.a = a;
    cb.cnt = 1;
    let real = cb.bind(cb);
    real.original = cb;
    return addHeapObject(real);
};

__exports.__wbindgen_closure_wrapper3791 = function(a, b, _ignored) {
    const f = wasm.__wbg_function_table.get(118);
    const d = wasm.__wbg_function_table.get(119);
    const cb = function(arg0) {
        this.cnt++;
        let a = this.a;
        this.a = 0;
        try {
            return f(a, b, addHeapObject(arg0));

        } finally {
            if (--this.cnt === 0) d(a, b);
            else this.a = a;

        }

    };
    cb.a = a;
    cb.cnt = 1;
    let real = cb.bind(cb);
    real.original = cb;
    return addHeapObject(real);
};

__exports.__wbindgen_object_clone_ref = function(idx) {
    return addHeapObject(getObject(idx));
};

__exports.__wbindgen_object_drop_ref = function(i) { dropObject(i); };

function init(module) {
    let result;
    const imports = { './package': __exports };

    if (module instanceof URL || typeof module === 'string' || module instanceof Request) {

        const response = fetch(module);
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            result = WebAssembly.instantiateStreaming(response, imports)
            .catch(e => {
                console.warn("`WebAssembly.instantiateStreaming` failed. Assuming this is because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);
                return response
                .then(r => r.arrayBuffer())
                .then(bytes => WebAssembly.instantiate(bytes, imports));
            });
        } else {
            result = response
            .then(r => r.arrayBuffer())
            .then(bytes => WebAssembly.instantiate(bytes, imports));
        }
    } else {

        result = WebAssembly.instantiate(module, imports)
        .then(result => {
            if (result instanceof WebAssembly.Instance) {
                return { instance: result, module };
            } else {
                return result;
            }
        });
    }
    return result.then(({instance, module}) => {
        wasm = instance.exports;
        init.__wbindgen_wasm_module = module;

        return wasm;
    });
}

self.wasm_bindgen = Object.assign(init, __exports);

})();
