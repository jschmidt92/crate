import type { RefObject } from "preact";
import { useEffect, useRef, useState } from "preact/hooks";
import { sendToArma } from "./bridge/host";
import { SystemMap3D } from "./components/SystemMap3D";
import { BankPage } from "./features/bank/BankPage";

type Page = "landing" | "about" | "bank";
type Section = "landing" | "map";

export function App() {
    const [currentPage, setCurrentPage] = useState<Page>("landing");
    const [activeSection, setActiveSection] = useState<Section>("landing");
    const [showScrollTop, setShowScrollTop] = useState(false);

    const landingRef = useRef<HTMLElement>(null);
    const mapRef = useRef<HTMLElement>(null);

    const openPage = (page: Page) => {
        setCurrentPage(page);
        setActiveSection("landing");

        window.scrollTo({
            top: 0,
            behavior: "smooth"
        });
    };

    const scrollToSection = (section: Section) => {
        setCurrentPage("landing");
        setActiveSection(section);

        requestAnimationFrame(() => {
            const target = {
                landing: landingRef.current,
                map: mapRef.current
            }[section];

            target?.scrollIntoView({ behavior: "smooth", block: "start" });
        });
    };

    const scrollToTop = () => {
        window.scrollTo({
            top: 0,
            behavior: "smooth"
        });
    };

    const openModule = (moduleId: string) => {
        if (moduleId === "bank") {
            openPage("bank");
            return;
        }

        sendToArma("ui::module_open_requested", { module: moduleId });
    };

    useEffect(() => {
        const updateScrollTopVisibility = () => {
            setShowScrollTop(window.scrollY > 120);
        };

        updateScrollTopVisibility();
        window.addEventListener("scroll", updateScrollTopVisibility, { passive: true });

        return () => {
            window.removeEventListener("scroll", updateScrollTopVisibility);
        };
    }, []);

    return (
        <div className="app-shell">
            <header className="topbar">
                <button
                    className="brand"
                    type="button"
                    aria-label="Forge landing page"
                    onClick={() => openPage("landing")}
                >
                    <span className="brand-mark" aria-hidden="true">
                        F
                    </span>
                    <span>Forge</span>
                </button>

                <nav className="nav" aria-label="Primary">
                    <button
                        className={currentPage === "landing" ? "active" : ""}
                        type="button"
                        onClick={() => openPage("landing")}
                    >
                        Landing
                    </button>

                    <button
                        className={currentPage === "about" ? "active" : ""}
                        type="button"
                        onClick={() => openPage("about")}
                    >
                        About
                    </button>

                    <button
                        className={currentPage === "bank" ? "active" : ""}
                        type="button"
                        onClick={() => openPage("bank")}
                    >
                        Bank
                    </button>
                </nav>
            </header>

            <main className="main-view" aria-labelledby="page-title">
                {currentPage === "landing" && (
                    <LandingPage
                        landingRef={landingRef}
                        mapRef={mapRef}
                        onOpenMap={() => scrollToSection("map")}
                        onOpenModule={openModule}
                        onScrollToSection={scrollToSection}
                    />
                )}

                {currentPage === "about" && <AboutPage />}
                {currentPage === "bank" && <BankPage />}
            </main>

            {showScrollTop && (
                <button
                    className="scroll-top-button"
                    type="button"
                    aria-label="Scroll to top"
                    onClick={scrollToTop}
                >
                    <svg width="20" height="20" viewBox="0 0 24 24" aria-hidden="true">
                        <path
                            d="M12 5l-7 7m7-7l7 7M12 5v14"
                            fill="none"
                            stroke="currentColor"
                            strokeWidth="2.5"
                            strokeLinecap="round"
                            strokeLinejoin="round"
                        />
                    </svg>
                </button>
            )}
        </div>
    );
}

function LandingPage({
    landingRef,
    mapRef,
    onOpenMap,
    onOpenModule,
    onScrollToSection
}: {
    landingRef: RefObject<HTMLElement>;
    mapRef: RefObject<HTMLElement>;
    onOpenMap: () => void;
    onOpenModule: (moduleId: string) => void;
    onScrollToSection: (section: Section) => void;
}) {
    return (
        <div className="content-stack">
            <section ref={landingRef} className="page-section hero-section">
                <div className="section-copy">
                    <div className="page-label">Landing</div>
                    <h1 id="page-title">Forge Operations</h1>
                    <p>
                        A focused command surface for player economy, organizations, storage,
                        services, and notifications.
                    </p>
                    <div className="actions">
                        <button className="primary-action" type="button" onClick={onOpenMap}>
                            Open Map Page
                        </button>

                        <button
                            className="secondary-action"
                            type="button"
                            onClick={() => onScrollToSection("map")}
                        >
                            Jump to Map Section
                        </button>

                        <button
                            className="secondary-action"
                            type="button"
                            onClick={() => sendToArma("ui::ping", { source: "landing" })}
                        >
                            Ping Arma
                        </button>
                    </div>
                </div>
            </section>

            <section ref={mapRef} className="page-section module-section" aria-labelledby="map-title">
                <div className="section-copy">
                    <div className="page-label">Map</div>
                    <h2 id="map-title">Feature Surface</h2>
                    <p>
                        Drag to rotate, scroll to zoom, and select a module to send a UI event back
                        through the Arma bridge.
                    </p>
                </div>
                <div className="module-panel">
                    <SystemMap3D onOpenModule={onOpenModule} />
                </div>
            </section>
        </div>
    );
}

function AboutPage() {
    return (
        <div className="content-stack">
            <section className="page-section about-layout" aria-labelledby="about-title">
                <div className="section-copy">
                    <div className="page-label">About</div>
                    <h1 id="page-title">About Forge</h1>
                    <p>
                        Forge is the mission framework layer that keeps persistent gameplay state,
                        feature workflows, and in-game interfaces moving through one consistent path.
                    </p>
                </div>

                <div className="info-list">
                    <div>
                        <span>UI runtime</span>
                        <strong>Preact</strong>
                    </div>
                    <div>
                        <span>Navigation</span>
                        <strong>In-memory pages + sections</strong>
                    </div>
                    <div>
                        <span>Target</span>
                        <strong>CT_WEBBROWSER</strong>
                    </div>
                </div>
            </section>
        </div>
    );
}
