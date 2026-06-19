type ForgeEventData = Record<string, unknown>;

declare global {
    interface Window {
        A3API?: {
            NavigateTo?: (url: string) => void;
            SendAlert?: (message: string) => void;
        };
    }
}

export function sendToArma(event: string, data: ForgeEventData = {}) {
    const payload = JSON.stringify({ event, data });

    if (window.A3API?.SendAlert) {
        window.A3API.SendAlert(payload);
        return true;
    }

    return false;
}

export function navigateWithArma(url: string) {
    if (window.A3API?.NavigateTo) {
        window.A3API.NavigateTo(url);
        return true;
    }

    window.location.href = url;
    return false;
}
