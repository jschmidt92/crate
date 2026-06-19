import { useEffect, useRef, useState } from "preact/hooks";
import * as THREE from "three";
import { sendToArma } from "../bridge/host";

type ModuleNode = {
    id: string;
    label: string;
    position: [number, number, number];
    icon:
        | "core"
        | "bank"
        | "org"
        | "garage"
        | "locker"
        | "services"
        | "store"
        | "cad"
        | "phone"
        | "tasks";
    core?: boolean;
};

const modules: ModuleNode[] = [
    { id: "core", label: "Core", icon: "core", position: [0, 0, 0], core: true },

    { id: "bank", label: "Bank", icon: "bank", position: [-2.7, 1.6, 0] },
    { id: "organization", label: "Org", icon: "org", position: [2.7, 1.6, 0] },

    { id: "garage", label: "Garage", icon: "garage", position: [-3.1, 0, 0] },
    { id: "services", label: "Services", icon: "services", position: [3.1, 0, 0] },

    { id: "locker", label: "Locker", icon: "locker", position: [-2.45, -1.55, 0] },
    { id: "store", label: "Store", icon: "store", position: [2.45, -1.55, 0] },

    { id: "cad", label: "CAD", icon: "cad", position: [-1.15, -2.45, 0] },
    { id: "phone", label: "Phone", icon: "phone", position: [1.15, -2.45, 0] },

    { id: "tasks", label: "Tasks", icon: "tasks", position: [0, 2.65, 0] }
];

const CARD_FACE_Z = 0.27;
const CONNECTOR_Z = -0.08;

type NodeMesh = THREE.Mesh<THREE.CylinderGeometry, THREE.MeshStandardMaterial> & {
    userData: {
        module: ModuleNode;
        baseScale: number;
        icon: THREE.Sprite;
    };
};

type Pulse = {
    mesh: THREE.Mesh<THREE.SphereGeometry, THREE.MeshBasicMaterial>;
    start: THREE.Vector3;
    end: THREE.Vector3;
    offset: number;
};

export function SystemMap3D({ onOpenModule }: { onOpenModule?: (moduleId: string) => void }) {
    const mountRef = useRef<HTMLDivElement>(null);
    const [selected, setSelected] = useState<ModuleNode>(modules[0]);
    const [fallback, setFallback] = useState(false);

    useEffect(() => {
        const mount = mountRef.current;
        if (!mount) {
            return;
        }

        let renderer: THREE.WebGLRenderer;
        try {
            renderer = new THREE.WebGLRenderer({
                antialias: true,
                alpha: true,
                preserveDrawingBuffer: true
            });
        } catch {
            setFallback(true);
            return;
        }

        const scene = new THREE.Scene();
        const camera = new THREE.PerspectiveCamera(42, 1, 0.1, 100);
        const group = new THREE.Group();
        const raycaster = new THREE.Raycaster();
        const pointer = new THREE.Vector2();
        const nodeMeshes: NodeMesh[] = [];
        const iconSprites: THREE.Sprite[] = [];
        const pulses: Pulse[] = [];
        const lineMaterial = new THREE.LineBasicMaterial({
            color: 0xd6c391,
            transparent: true,
            opacity: 0.38
        });
        const startedAt = performance.now();
        const targetScaleVector = new THREE.Vector3(1, 1, 1);

        let hovered: NodeMesh | null = null;
        let selectedId = selected.id;
        let rotationX = -0.24;
        let rotationY = 0.62;
        let targetRotationX = rotationX;
        let targetRotationY = rotationY;
        let distance = 7.6;
        let targetDistance = distance;
        let dragging = false;
        let lastX = 0;
        let lastY = 0;
        let animationFrame = 0;

        renderer.setPixelRatio(Math.min(window.devicePixelRatio || 1, 2));
        renderer.setClearColor(0x000000, 0);
        renderer.domElement.className = "system-map-canvas";
        mount.appendChild(renderer.domElement);

        scene.add(group);
        scene.add(new THREE.AmbientLight(0xe4e4e7, 1.35));

        const keyLight = new THREE.DirectionalLight(0xffe4aa, 2.2);
        keyLight.position.set(3, 5, 4);
        scene.add(keyLight);

        const fillLight = new THREE.DirectionalLight(0x8bb8ff, 0.75);
        fillLight.position.set(-4, -2, 3);
        scene.add(fillLight);

        for (const module of modules) {
            if (!module.core) {
                const start = getConnectionPoint(modules[0]);
                const end = getConnectionPoint(module);
                const geometry = new THREE.BufferGeometry().setFromPoints([start, end]);
                const line = new THREE.Line(geometry, lineMaterial);
                group.add(line);

                const pulse = new THREE.Mesh(
                    new THREE.SphereGeometry(0.055, 16, 12),
                    new THREE.MeshBasicMaterial({
                        color: 0xf2cf78,
                        transparent: true,
                        opacity: 0.74
                    })
                );
                group.add(pulse);
                pulses.push({ mesh: pulse, start, end, offset: pulses.length * 0.17 });
            }
        }

        for (const module of modules) {
            const radius = module.core ? 0.72 : 0.58;
            const geometry = new THREE.CylinderGeometry(radius, radius, module.core ? 0.18 : 0.14, 6);
            geometry.rotateX(Math.PI / 2);
            const material = new THREE.MeshStandardMaterial({
                color: module.core ? 0xd6a84f : 0x18181b,
                emissive: module.core ? 0x7a520c : 0x27272a,
                roughness: 0.36,
                metalness: 0.34
            });
            const mesh = new THREE.Mesh(geometry, material) as NodeMesh;
            mesh.position.set(...module.position);
            mesh.rotation.z = Math.PI / 6;

            const icon = createModuleCard(module);
            icon.position.set(module.position[0], module.position[1], CARD_FACE_Z);
            group.add(icon);
            iconSprites.push(icon);

            mesh.userData = {
                module,
                baseScale: module.core ? 1.08 : 1,
                icon
            };
            group.add(mesh);
            nodeMeshes.push(mesh);
        }

        // const grid = new THREE.GridHelper(7, 10, 0x52525b, 0x27272a);
        // grid.rotation.x = Math.PI / 2;
        // grid.position.z = -0.62;
        // group.add(grid);

        const resize = () => {
            const rect = mount.getBoundingClientRect();
            const width = Math.max(1, Math.floor(rect.width));
            const height = Math.max(1, Math.floor(rect.height));
            renderer.setSize(width, height, false);
            camera.aspect = width / height;
            camera.updateProjectionMatrix();
        };

        const setPointer = (event: PointerEvent) => {
            const rect = renderer.domElement.getBoundingClientRect();
            pointer.x = ((event.clientX - rect.left) / rect.width) * 2 - 1;
            pointer.y = -((event.clientY - rect.top) / rect.height) * 2 + 1;
        };

        const updateHover = (event: PointerEvent) => {
            setPointer(event);
            raycaster.setFromCamera(pointer, camera);
            const hit = raycaster.intersectObjects(nodeMeshes, false)[0]?.object as NodeMesh | undefined;
            hovered = hit ?? null;
            renderer.domElement.style.cursor = hovered ? "pointer" : dragging ? "grabbing" : "grab";
        };

        const onPointerDown = (event: PointerEvent) => {
            dragging = true;
            lastX = event.clientX;
            lastY = event.clientY;
            renderer.domElement.setPointerCapture(event.pointerId);
            renderer.domElement.style.cursor = "grabbing";
        };

        const onPointerMove = (event: PointerEvent) => {
            if (dragging) {
                const dx = event.clientX - lastX;
                const dy = event.clientY - lastY;
                targetRotationY += dx * 0.008;
                targetRotationX = clamp(targetRotationX + dy * 0.006, -0.95, 0.72);
                lastX = event.clientX;
                lastY = event.clientY;
            }
            updateHover(event);
        };

        const onPointerUp = (event: PointerEvent) => {
            dragging = false;
            renderer.domElement.releasePointerCapture(event.pointerId);
            updateHover(event);
        };

        const onClick = () => {
            if (!hovered) {
                return;
            }
            selectedId = hovered.userData.module.id;
            setSelected(hovered.userData.module);
            sendToArma("ui::module_selected", {
                module: hovered.userData.module.id
            });
            onOpenModule?.(hovered.userData.module.id);
        };

        const onWheel = (event: WheelEvent) => {
            event.preventDefault();
            targetDistance = clamp(targetDistance + event.deltaY * 0.0035, 6, 12);
        };

        const animate = () => {
            const elapsed = (performance.now() - startedAt) / 1000;
            rotationX += (targetRotationX - rotationX) * 0.12;
            rotationY += (targetRotationY - rotationY) * 0.12;
            distance += (targetDistance - distance) * 0.12;
            group.rotation.x = rotationX;
            group.rotation.y = rotationY + Math.sin(elapsed * 0.35) * 0.035;
            camera.position.set(0, 0, distance);
            camera.lookAt(0, 0, 0);

            for (const mesh of nodeMeshes) {
                const active = mesh.userData.module.id === selectedId;
                const isHovered = hovered === mesh;
                const targetScale = mesh.userData.baseScale * (active ? 1.18 : isHovered ? 1.1 : 1);
                targetScaleVector.set(targetScale, targetScale, targetScale);
                mesh.scale.lerp(targetScaleVector, 0.18);
                mesh.material.emissiveIntensity = active ? 1.1 : isHovered ? 0.72 : 0.34;
                mesh.userData.icon.scale.lerp(
                    targetScaleVector.set(active ? 1.18 : isHovered ? 1.08 : 1, active ? 1.18 : isHovered ? 1.08 : 1, 1),
                    0.18
                );
            }

            for (const pulse of pulses) {
                const progress = (elapsed * 0.32 + pulse.offset) % 1;
                pulse.mesh.position.lerpVectors(pulse.start, pulse.end, progress);
                pulse.mesh.material.opacity = 0.25 + Math.sin(progress * Math.PI) * 0.58;
                const pulseScale = 0.7 + Math.sin(progress * Math.PI) * 0.55;
                pulse.mesh.scale.setScalar(pulseScale);
            }

            for (const sprite of iconSprites) {
                sprite.quaternion.copy(camera.quaternion);
            }

            renderer.render(scene, camera);
            animationFrame = requestAnimationFrame(animate);
        };

        resize();
        animate();

        renderer.domElement.addEventListener("pointerdown", onPointerDown);
        renderer.domElement.addEventListener("pointermove", onPointerMove);
        renderer.domElement.addEventListener("pointerup", onPointerUp);
        renderer.domElement.addEventListener("pointercancel", onPointerUp);
        renderer.domElement.addEventListener("click", onClick);
        renderer.domElement.addEventListener("wheel", onWheel, { passive: false });
        window.addEventListener("resize", resize);

        return () => {
            cancelAnimationFrame(animationFrame);
            window.removeEventListener("resize", resize);
            renderer.domElement.removeEventListener("pointerdown", onPointerDown);
            renderer.domElement.removeEventListener("pointermove", onPointerMove);
            renderer.domElement.removeEventListener("pointerup", onPointerUp);
            renderer.domElement.removeEventListener("pointercancel", onPointerUp);
            renderer.domElement.removeEventListener("click", onClick);
            renderer.domElement.removeEventListener("wheel", onWheel);
            mount.removeChild(renderer.domElement);

            scene.traverse((object) => {
                if (object instanceof THREE.Mesh) {
                    object.geometry.dispose();
                    disposeMaterial(object.material);
                }
                if (object instanceof THREE.Sprite) {
                    object.material.map?.dispose();
                    object.material.dispose();
                }
                if (object instanceof THREE.Line) {
                    object.geometry.dispose();
                }
            });
            lineMaterial.dispose();
            renderer.dispose();
        };
    }, []);

    if (fallback) {
        return <StaticSystemMap selected={selected} onSelect={setSelected} onOpenModule={onOpenModule} />;
    }

    return (
        <div className="system-map system-map-3d" aria-label="Forge module map">
            <div ref={mountRef} className="system-map-stage" />
            <div className="system-map-readout">
                <span>Selected</span>
                <strong>{selected.label}</strong>
            </div>
        </div>
    );
}

function StaticSystemMap({
    selected,
    onSelect,
    onOpenModule
}: {
    selected: ModuleNode;
    onSelect: (module: ModuleNode) => void;
    onOpenModule?: (moduleId: string) => void;
}) {
    return (
        <div className="system-map system-map-fallback" aria-label="Forge module map">
            {modules.map((module) => (
                <button
                    key={module.id}
                    className={`system-map-fallback-node ${module.core ? "core" : ""} ${selected.id === module.id ? "active" : ""
                        }`}
                    type="button"
                    onClick={() => {
                        onSelect(module);
                        sendToArma("ui::module_selected", { module: module.id });
                        onOpenModule?.(module.id);
                    }}
                >
                    {module.label}
                </button>
            ))}
        </div>
    );
}

function createModuleCard(module: ModuleNode) {
    const canvas = document.createElement("canvas");
    canvas.width = 320;
    canvas.height = 192;
    const context = canvas.getContext("2d");
    if (!context) {
        throw new Error("Unable to create module card canvas.");
    }

    const accent = module.core ? "#14110a" : "#d6a84f";
    const text = module.core ? "#14110a" : "#f4f4f5";
    const line = module.core ? "rgba(20, 17, 10, 0.45)" : "rgba(214, 168, 79, 0.68)";

    context.clearRect(0, 0, canvas.width, canvas.height);
    context.save();
    context.translate(canvas.width / 2, 68);
    drawModuleIcon(context, module.icon, accent, line);
    context.restore();

    context.fillStyle = text;
    context.font = "800 30px Arial";
    context.textAlign = "center";
    context.textBaseline = "middle";
    context.fillText(module.label.toUpperCase(), canvas.width / 2, 136);

    context.strokeStyle = line;
    context.lineWidth = 3;
    context.beginPath();
    context.moveTo(116, 164);
    context.lineTo(204, 164);
    context.stroke();

    const texture = new THREE.CanvasTexture(canvas);
    texture.colorSpace = THREE.SRGBColorSpace;
    const material = new THREE.SpriteMaterial({
        map: texture,
        transparent: true,
        depthTest: false
    });
    const sprite = new THREE.Sprite(material);
    sprite.scale.set(1.18, 0.72, 1);
    return sprite;
}

function drawModuleIcon(
    context: CanvasRenderingContext2D,
    icon: ModuleNode["icon"],
    accent: string,
    line: string
) {
    context.strokeStyle = accent;
    context.fillStyle = accent;
    context.lineWidth = 6;
    context.lineCap = "round";
    context.lineJoin = "round";

    if (icon === "core") {
        context.save();
        context.fillStyle = accent;

        for (let index = 0; index < 8; index += 1) {
            context.save();
            context.rotate((Math.PI * 2 * index) / 8);
            context.beginPath();
            context.roundRect(-7, -48, 14, 18, 2);
            context.fill();
            context.restore();
        }

        context.lineWidth = 15;
        context.beginPath();
        context.arc(0, 0, 28, 0, Math.PI * 2);
        context.stroke();

        context.fillStyle = "#d6a84f";
        context.beginPath();
        context.arc(0, 0, 11, 0, Math.PI * 2);
        context.fill();

        context.lineWidth = 5;
        context.beginPath();
        context.arc(0, 0, 11, 0, Math.PI * 2);
        context.stroke();

        context.restore();

        return;
    }

    if (icon === "bank") {
        context.beginPath();
        context.moveTo(-42, -18);
        context.lineTo(0, -42);
        context.lineTo(42, -18);
        context.closePath();
        context.stroke();
        for (const x of [-28, -9, 9, 28]) {
            context.beginPath();
            context.moveTo(x, -12);
            context.lineTo(x, 30);
            context.stroke();
        }
        context.beginPath();
        context.moveTo(-44, 38);
        context.lineTo(44, 38);
        context.stroke();
        return;
    }

    if (icon === "org") {
        for (const point of [
            [0, -36],
            [-38, 28],
            [38, 28]
        ] as const) {
            context.beginPath();
            context.arc(point[0], point[1], 12, 0, Math.PI * 2);
            context.fill();
        }
        context.strokeStyle = line;
        context.beginPath();
        context.moveTo(0, -24);
        context.lineTo(-29, 20);
        context.moveTo(0, -24);
        context.lineTo(29, 20);
        context.moveTo(-26, 28);
        context.lineTo(26, 28);
        context.stroke();
        return;
    }

    if (icon === "garage") {
        context.save();
        context.lineWidth = 6;
        context.lineCap = "round";
        context.lineJoin = "round";

        // main car body
        context.beginPath();
        context.roundRect(-42, -4, 84, 30, 8);
        context.stroke();

        // simple cabin
        context.beginPath();
        context.moveTo(-22, -4);
        context.lineTo(-10, -24);
        context.lineTo(14, -24);
        context.lineTo(28, -4);
        context.stroke();

        // windshield divider
        context.beginPath();
        context.moveTo(2, -24);
        context.lineTo(2, -4);
        context.stroke();

        // wheels
        context.beginPath();
        context.arc(-24, 28, 8, 0, Math.PI * 2);
        context.arc(24, 28, 8, 0, Math.PI * 2);
        context.stroke();

        // wheel centers
        context.beginPath();
        context.arc(-24, 28, 2.5, 0, Math.PI * 2);
        context.arc(24, 28, 2.5, 0, Math.PI * 2);
        context.fill();

        // headlights / taillights
        context.beginPath();
        context.moveTo(-39, 8);
        context.lineTo(-34, 8);
        context.moveTo(34, 8);
        context.lineTo(39, 8);
        context.stroke();

        context.restore();
        return;
    }

    if (icon === "locker") {
        context.beginPath();
        context.roundRect(-36, -42, 72, 84, 8);
        context.stroke();
        context.beginPath();
        context.moveTo(-16, -20);
        context.lineTo(16, -20);
        context.moveTo(-16, 0);
        context.lineTo(16, 0);
        context.moveTo(22, 18);
        context.lineTo(22, 30);
        context.stroke();
        return;
    }

    if (icon === "store") {
        context.save();
        context.lineWidth = 6;
        context.lineCap = "round";
        context.lineJoin = "round";

        // cart handle
        context.beginPath();
        context.moveTo(-42, -28);
        context.lineTo(-30, -28);
        context.lineTo(-22, 12);
        context.stroke();

        // cart basket
        context.beginPath();
        context.moveTo(-20, -16);
        context.lineTo(34, -16);
        context.lineTo(26, 12);
        context.lineTo(-14, 12);
        context.closePath();
        context.stroke();

        // basket slats
        context.beginPath();
        context.moveTo(-10, -16);
        context.lineTo(-6, 12);
        context.moveTo(6, -16);
        context.lineTo(6, 12);
        context.moveTo(22, -16);
        context.lineTo(18, 12);
        context.stroke();

        // wheels
        context.beginPath();
        context.arc(-10, 30, 6, 0, Math.PI * 2);
        context.arc(22, 30, 6, 0, Math.PI * 2);
        context.stroke();

        // wheel supports
        context.beginPath();
        context.moveTo(-10, 12);
        context.lineTo(-10, 24);
        context.moveTo(22, 12);
        context.lineTo(22, 24);
        context.stroke();

        context.restore();
        return;
    }

    if (icon === "cad") {
        // monitor
        context.beginPath();
        context.roundRect(-38, -34, 76, 48, 5);
        context.stroke();

        // dispatch lines
        context.beginPath();
        context.moveTo(-24, -18);
        context.lineTo(8, -18);
        context.moveTo(-24, -4);
        context.lineTo(22, -4);
        context.moveTo(-24, 10);
        context.lineTo(2, 10);
        context.stroke();

        // alert marker
        context.beginPath();
        context.arc(23, -18, 6, 0, Math.PI * 2);
        context.fill();

        // monitor stand
        context.beginPath();
        context.moveTo(0, 14);
        context.lineTo(0, 30);
        context.moveTo(-18, 30);
        context.lineTo(18, 30);
        context.stroke();

        return;
    }

    if (icon === "phone") {
        context.save();
        context.lineWidth = 6;
        context.lineCap = "round";
        context.lineJoin = "round";

        // phone handset
        context.beginPath();
        context.moveTo(-36, -12);
        context.bezierCurveTo(-30, -34, -6, -42, 18, -34);
        context.bezierCurveTo(34, -28, 42, -14, 34, 0);
        context.stroke();

        // left earpiece
        context.beginPath();
        context.roundRect(-42, -10, 20, 24, 6);
        context.stroke();

        // right mouthpiece
        context.beginPath();
        context.roundRect(22, -2, 20, 24, 6);
        context.stroke();

        // cord curl
        context.lineWidth = 5;
        context.beginPath();
        context.moveTo(-6, 10);
        context.bezierCurveTo(-18, 20, -8, 30, 4, 22);
        context.bezierCurveTo(16, 14, 26, 26, 10, 36);
        context.stroke();

        context.restore();
        return;
    }

    if (icon === "services") {
        context.save();
        context.lineWidth = 6;
        context.lineCap = "round";
        context.lineJoin = "round";

        // Service station pump.
        context.beginPath();
        context.roundRect(-34, -38, 42, 70, 6);
        context.stroke();

        context.beginPath();
        context.roundRect(-24, -28, 22, 18, 3);
        context.stroke();

        context.beginPath();
        context.moveTo(-40, 38);
        context.lineTo(14, 38);
        context.stroke();

        context.beginPath();
        context.moveTo(8, -24);
        context.bezierCurveTo(32, -24, 28, 1, 40, 1);
        context.bezierCurveTo(50, 1, 50, -16, 38, -16);
        context.stroke();

        context.beginPath();
        context.moveTo(36, -22);
        context.lineTo(48, -10);
        context.lineTo(42, -4);
        context.stroke();

        context.fillStyle = accent;
        context.beginPath();
        context.arc(40, 1, 5, 0, Math.PI * 2);
        context.fill();

        context.restore();
        return;
    }

    if (icon === "tasks") {
        // clipboard
        context.beginPath();
        context.roundRect(-30, -34, 60, 72, 6);
        context.stroke();

        // clip
        context.beginPath();
        context.roundRect(-14, -42, 28, 14, 4);
        context.stroke();

        // check 1
        context.beginPath();
        context.moveTo(-18, -12);
        context.lineTo(-10, -4);
        context.lineTo(2, -18);
        context.stroke();

        // line 1
        context.beginPath();
        context.moveTo(10, -10);
        context.lineTo(22, -10);
        context.stroke();

        // check 2
        context.beginPath();
        context.moveTo(-18, 14);
        context.lineTo(-10, 22);
        context.lineTo(2, 8);
        context.stroke();

        // line 2
        context.beginPath();
        context.moveTo(10, 16);
        context.lineTo(22, 16);
        context.stroke();

        return;
    }

    context.beginPath();
    context.moveTo(-30, -34);
    context.lineTo(2, -2);
    context.moveTo(-2, 2);
    context.lineTo(34, 38);
    context.stroke();
    context.beginPath();
    context.arc(-35, -39, 9, 0, Math.PI * 2);
    context.stroke();
    context.beginPath();
    context.moveTo(24, -30);
    context.lineTo(42, -12);
    context.moveTo(42, -30);
    context.lineTo(24, -12);
    context.stroke();
}

function disposeMaterial(material: THREE.Material | THREE.Material[]) {
    if (Array.isArray(material)) {
        for (const item of material) {
            item.dispose();
        }
        return;
    }
    material.dispose();
}

function clamp(value: number, min: number, max: number) {
    return Math.min(max, Math.max(min, value));
}

function getConnectionPoint(module: ModuleNode) {
    const [x, y] = module.position;
    return new THREE.Vector3(x, y, CONNECTOR_Z);
}
