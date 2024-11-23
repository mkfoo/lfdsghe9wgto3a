class WasmGame {
    constructor() {
        this.events = [];
        this.touches = {};
    }

    imports = {
        env: {
            get_keydown: () => {
                return this.events.shift() || UNKNOWN;
            },
            create_texture: (ptr, width, height) => {
                if (ptr) {
                    const arr = new Uint8ClampedArray(
                        this.exports.memory.buffer,
                        ptr,
                        width * height * 4,
                    );
                    return this.renderer.createTexture(arr, width, height);
                } else {
                    return this.renderer.createTexture(null, width, height);
                }
            },
            set_render_target: (idx) => {
                this.renderer.setRenderTarget(idx);
            },
            set_render_source: (idx) => {
                this.renderer.setRenderSource(idx);
            },
            set_line_color: (idx) => {
                this.renderer.setLineColor(idx);
            },
            set_fill_color: (idx) => {
                this.renderer.setFillColor(idx);
            },
            set_rect_size: (sw, sh, dw, dh) => {
                this.renderer.setRectSize(sw, sh, dw, dh);
            },
            render_copy: (sx, sy, dx, dy) => {
                this.renderer.renderCopy(sx, sy, dx, dy);
            },
            fill_rect: (x, y, w, h) => {
                this.renderer.fillRect(x, y, w, h);
            },
            toggle_scale_factor: () => {
                this.renderer.toggleScaleFactor();
            },
            begin_path: () => {
                this.renderer.target.beginPath();
            },
            close_path: () => {
                this.renderer.target.closePath();
            },
            move_to: (x, y) => {
                this.renderer.target.moveTo(x, y);
            },
            line_to: (x, y) => {
                this.renderer.target.lineTo(x, y);
            },
            stroke: () => {
                this.renderer.target.stroke();
            },
            set_color_mod: (idx) => {
                this.renderer.setColorMod(idx);
            },
        },
    };

    async main() {
        const mod = await WebAssembly.compileStreaming(fetch("main.wasm"));
        const inst = await WebAssembly.instantiate(mod, this.imports);
        this.exports = inst.exports;
        this.update = inst.exports.update;
        const scale = this.getScaleFactor();
        this.renderer = new CanvasRenderer(scale);
        const err = this.exports.init(performance.now());

        if (err) {
            throw new Error("init failed");
        }

        document.addEventListener("keydown", (e) => {
            const keycode = KEYMAP[e.key];

            if (keycode) {
                e.preventDefault();

                if (!e.repeat) {
                    this.events.push(keycode);
                }
            }
        });

        window.addEventListener("resize", (e) => {
            const sf = this.getScaleFactor();
            this.renderer.setScaleFactor(sf);
        });

        document.addEventListener("touchstart", (e) => {
            this.handleStart(e);
        }, { passive: false });

        document.addEventListener("touchmove", (e) => {
            this.handleMove(e);
        }, { passive: false });

        document.addEventListener("touchend", (e) => {
            this.handleEnd(e);
        });

        document.addEventListener("touchcancel", (e) => {
            this.handleEnd(e);
        });

        const nextFrame = (timestamp) => {
            if (!this.update(timestamp)) {
                window.requestAnimationFrame(nextFrame);
            } else {
                const canvas = document.querySelector("canvas");
                document.body.removeChild(canvas);
            }
        };

        window.requestAnimationFrame(nextFrame);
    }

    getScaleFactor() {
        const w = window.visualViewport.width;
        const h = window.visualViewport.height;

        if (w > h) {
            return Math.floor(h / ORIG_HEIGHT * 10) / 10;
        } else {
            return Math.floor(w / ORIG_WIDTH * 10) / 10;
        }
    }

    handleStart(e) {
        e.preventDefault();

        for (let t of e.changedTouches) {
            this.touches[t.identifier] = t;
        }

        if (!this.tap) {
            this.tap = true;
            setTimeout(() => {
                this.tap = false;
            }, 250);
        } else {
            this.events.push(KEYMAP[" "]);
            this.tap = false;
        }
    }

    handleMove(e) {
        e.preventDefault();

        let key = null;

        for (let t of e.changedTouches) {
            const m = 25;
            const prev = this.touches[t.identifier];
            const dx = t.pageX - prev.pageX;
            const dy = t.pageY - prev.pageY;

            if (dx > m) {
                key = "ArrowRight";
            } else if (dy > m) {
                key = "ArrowDown";
            } else if (dx < -m) {
                key = "ArrowLeft";
            } else if (dy < -m) {
                key = "ArrowUp";
            }

            this.touches[t.identifier] = t;
        }

        if (key) {
            this.events.push(KEYMAP[key]);
        }
    }

    handleEnd(e) {
        e.preventDefault();

        for (let t of e.changedTouches) {
            delete this.touches[t.identifier];
        }
    }
}

class CanvasRenderer {
    constructor(scale) {
        this.initCanvas();
        this.loadPalette();
        this.setScaleFactor(scale);
    }

    initCanvas() {
        const canvas = document.createElement("canvas");
        const ctx = canvas.getContext("2d", { alpha: false });

        if (!ctx) {
            const p = document.createElement("p");
            p.textContent =
                "Sorry, your browser does not support canvas rendering.";
            document.body.appendChild(p);
            throw new Error("Could not create 2D context.");
        }

        ctx.canvas.width = ORIG_WIDTH;
        ctx.canvas.height = ORIG_HEIGHT;
        ctx.imageSmoothingEnabled = false;
        document.body.appendChild(canvas);
        this.contexts = [ctx];
        this.target = this.contexts[0];
    }

    loadPalette() {
        const p = PALETTE;
        this.cssPalette = [];

        for (let i = 0; i < p.length; i += 4) {
            this.cssPalette.push(
                `rgba(${p[i + 0]},${p[i + 1]},${p[i + 2]},${p[i + 3]})`,
            );
        }
    }

    createTexture(arr, width, height) {
        try {
            const w = width || ORIG_WIDTH;
            const h = height || ORIG_HEIGHT;
            const canvas = document.createElement("canvas");
            const ctx = canvas.getContext("2d");
            ctx.canvas.width = w;
            ctx.canvas.height = h;
            ctx.imageSmoothingEnabled = false;

            if (arr) {
                const img = new ImageData(arr, w, h);
                ctx.putImageData(img, 0, 0);
            }

            this.contexts.push(ctx);
            return this.contexts.length - 1;
        } catch (err) {
            console.log(err.message);
            return 0;
        }
    }

    setRenderTarget(idx) {
        this.target = this.contexts[idx];
    }

    setRenderSource(idx) {
        this.source = this.contexts[idx].canvas;
    }


    setRectSize(sw, sh, dw, dh) {
        this.sw = sw;
        this.sh = sh;
        this.dw = dw;
        this.dh = dh;
    }

    renderCopy(sx, sy, dx, dy) {
        this.target.drawImage(
            this.source,
            sx,
            sy,
            this.sw,
            this.sh,
            dx,
            dy,
            this.dw,
            this.dh,
        );
    }

    setLineColor(idx) {
        this.target.strokeStyle = this.cssPalette[idx];
    }

    setFillColor(idx) {
        this.target.fillStyle = this.cssPalette[idx];
    }

    setColorMod(idx) {
        this.setFillColor(idx);
        this.target.globalCompositeOperation = "source-in";
        this.fillRect(0, 0, 0, 0);
    }

    fillRect(x, y, width, height) {
        const defaultW = this.target.canvas.width;
        const defaultH = this.target.canvas.height;
        const w = width || defaultW;
        const h = height || defaultH;
        this.target.fillRect(x, y, w, h);
    }

    toggleScaleFactor() {
        this.setScaleFactor((Math.floor(this.scaleFactor) + 1) % 11);
    }

    setScaleFactor(n) {
        const sf = n || 1;
        const w = window.visualViewport.width;
        const h = window.visualViewport.height;
        const margw = Math.floor((w - ORIG_WIDTH * sf) / 2);
        const margh = Math.floor((h - ORIG_HEIGHT * sf) / 2);
        const ctx = this.contexts[0];
        ctx.canvas.style =
            `transform-origin: 0 0;transform: scale(${sf});--margw: ${margw}px;--margh: ${margh}px;`;
        this.scaleFactor = sf;
    }
}

function chromeCompatibilityHack() {
    const game = new WasmGame();
    let ctx = new AudioContext();

    setTimeout(() => {
        if (ctx.state === "suspended") {
            const btn = document.createElement("button");
            btn.textContent = "Click to play";
            document.body.appendChild(btn);
            ctx = null;

            btn.onclick = (_) => {
                document.body.removeChild(btn);
                game.main();
            };
        } else {
            ctx = null;
            game.main();
        }
    }, 500);
}

chromeCompatibilityHack();
