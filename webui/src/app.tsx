import { useEffect, useRef, useState } from "preact/hooks";
import { Maximize2, Minus, X } from "lucide-preact";
import { sendToArma } from "./bridge/host";
import { BankPage } from "./features/bank/BankPage";

type Theme = "dark" | "light";

export function App() {
    const [showScrollTop, setShowScrollTop] = useState(false);
    const [theme, setTheme] = useState<Theme>(readStoredTheme);
    const mainViewRef = useRef<HTMLElement>(null);

    const scrollToTop = () => {
        mainViewRef.current?.scrollTo({
            top: 0,
            behavior: "smooth"
        });
    };

    useEffect(() => {
        document.documentElement.setAttribute("data-theme", theme);
        storeTheme(theme);
    }, [theme]);

    const toggleTheme = () => {
        setTheme((prev) => (prev === "dark" ? "light" : "dark"));
    };

    useEffect(() => {
        const mainView = mainViewRef.current;
        if (!mainView) {
            return;
        }

        const updateScrollTopVisibility = () => {
            setShowScrollTop(mainView.scrollTop > 120);
        };

        updateScrollTopVisibility();
        mainView.addEventListener("scroll", updateScrollTopVisibility, { passive: true });

        return () => {
            mainView.removeEventListener("scroll", updateScrollTopVisibility);
        };
    }, []);

    return (
        <div className="app-shell">
            <header className="topbar">
                <div className="window-title">
                    <span className="brand-mark" aria-hidden="true">
                        F
                    </span>
                    <span>FORGE Bank</span>
                </div>

                <div className="window-controls" aria-label="Window controls">
                    <button type="button" aria-label="Minimize" title="Minimize unavailable" disabled>
                        <Minus size={17} strokeWidth={1.8} aria-hidden="true" />
                    </button>
                    <button type="button" aria-label="Maximize" title="Maximize unavailable" disabled>
                        <Maximize2 size={15} strokeWidth={1.8} aria-hidden="true" />
                    </button>
                    <button
                        className="close-control"
                        type="button"
                        aria-label="Close"
                        title="Close"
                        onClick={() => sendToArma("ui::close")}
                    >
                        <X size={18} strokeWidth={1.8} aria-hidden="true" />
                    </button>
                </div>
            </header>

            <main ref={mainViewRef} className="main-view" aria-labelledby="page-title">
                <BankPage theme={theme} toggleTheme={toggleTheme} />
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

function readStoredTheme(): Theme {
    try {
        const stored = window.localStorage.getItem("theme");
        return stored === "light" || stored === "dark" ? stored : "dark";
    } catch {
        return "dark";
    }
}

function storeTheme(theme: Theme) {
    try {
        window.localStorage.setItem("theme", theme);
    } catch {
        // Arma's opaque WebBrowser origin does not expose persistent storage.
    }
}
