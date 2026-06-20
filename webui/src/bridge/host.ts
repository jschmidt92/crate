type ForgeEventData = Record<string, unknown>;

type ForgeHostResponse<T = unknown> = {
    requestId: string;
    event: string;
    ok: boolean;
    data: T;
    error: string;
};

type PendingRequest = {
    resolve: (value: unknown) => void;
    reject: (reason: Error) => void;
    timeout: number;
};

declare global {
    interface Window {
        A3API?: {
            NavigateTo?: (url: string) => void;
            SendAlert?: (message: string) => void;
        };
        forgeHostReceive?: (response: ForgeHostResponse) => void;
    }
}

const pendingRequests = new Map<string, PendingRequest>();
let requestSequence = 0;

type PushListener = (data: any) => void;
const pushListeners = new Map<string, Set<PushListener>>();

export function onPushEvent(event: string, listener: PushListener) {
    let listeners = pushListeners.get(event);
    if (!listeners) {
        listeners = new Set<PushListener>();
        pushListeners.set(event, listeners);
    }
    listeners.add(listener);
}

export function offPushEvent(event: string, listener: PushListener) {
    const listeners = pushListeners.get(event);
    if (listeners) {
        listeners.delete(listener);
        if (listeners.size === 0) {
            pushListeners.delete(event);
        }
    }
}

window.forgeHostReceive = (response) => {
    if (response.requestId) {
        const pending = pendingRequests.get(response.requestId);
        if (pending) {
            window.clearTimeout(pending.timeout);
            pendingRequests.delete(response.requestId);

            if (response.ok) {
                pending.resolve(response.data);
            } else {
                pending.reject(new Error(normalizeError(response.error)));
            }
            return;
        }
    }

    if (response.event) {
        const listeners = pushListeners.get(response.event);
        if (listeners) {
            for (const listener of listeners) {
                try {
                    listener(response.data);
                } catch (err) {
                    console.error(`Error in push listener for event ${response.event}:`, err);
                }
            }
        }
    }
};

export function sendToArma(event: string, data: ForgeEventData = {}) {
    if (!window.A3API?.SendAlert) {
        return false;
    }

    window.A3API.SendAlert(JSON.stringify({ event, data }));
    return true;
}

export function requestFromArma<T>(event: string, data: ForgeEventData = {}): Promise<T> {
    if (!window.A3API?.SendAlert) {
        return Promise.reject(new Error("Arma bridge unavailable"));
    }

    const requestId = `${Date.now()}-${++requestSequence}`;
    const payload = JSON.stringify({ requestId, event, data });

    return new Promise<T>((resolve, reject) => {
        const timeout = window.setTimeout(() => {
            pendingRequests.delete(requestId);
            reject(new Error("The bank request timed out"));
        }, 10_000);

        pendingRequests.set(requestId, {
            resolve: resolve as (value: unknown) => void,
            reject,
            timeout
        });
        window.A3API?.SendAlert?.(payload);
    });
}

export function navigateWithArma(url: string) {
    if (window.A3API?.NavigateTo) {
        window.A3API.NavigateTo(url);
        return true;
    }

    window.location.href = url;
    return false;
}

function normalizeError(error: string) {
    return error.startsWith("Error: ") ? error.slice(7) : error || "Bank request failed";
}
