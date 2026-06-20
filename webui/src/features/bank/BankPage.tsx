import { useCallback, useEffect, useState } from "preact/hooks";
import { onPushEvent, offPushEvent, requestFromArma } from "../../bridge/host";
import "./BankPage.css";

import {
    Coins,
    Landmark,
    TrendingUp,
    ShieldCheck,
    Users,
    Send,
    RefreshCw,
    AlertCircle,
    Info,
    Key,
    Sun,
    Moon
} from "lucide-preact";

type OrgRole = "ceo" | "member";

type BankTransaction = {
    id: string;
    amount: string;
    description: string;
    created_at: string;
};

type BankProfile = {
    uid: string;
    cash: string;
    account: {
        id: string;
        balance: string;
    };
    pending_earnings: string;
    pin_set: boolean;
    transactions: BankTransaction[];
};

type Organization = {
    name: string;
    bank: string;
    members: Array<{
        uid: string;
        role: OrgRole;
    }>;
};

type BankSnapshot = {
    profile: BankProfile;
    organization: Organization | Record<string, never>;
};

type BankMutation = {
    profile: BankProfile;
};

export function BankPage({ theme, toggleTheme }: { theme: "dark" | "light"; toggleTheme: () => void }) {
    const [movementAmount, setMovementAmount] = useState("250");
    const [transferAmount, setTransferAmount] = useState("250");
    const [transferTarget, setTransferTarget] = useState("");
    const [profile, setProfile] = useState<BankProfile | null>(null);
    const [organization, setOrganization] = useState<Organization | null>(null);
    const [currentPin, setCurrentPin] = useState("");
    const [newPin, setNewPin] = useState("");
    const [confirmPin, setConfirmPin] = useState("");
    const [loading, setLoading] = useState(true);
    const [busy, setBusy] = useState(false);
    const [error, setError] = useState("");

    const loadSnapshot = useCallback(async () => {
        setLoading(true);
        setError("");
        try {
            const snapshot = await requestFromArma<BankSnapshot>("bank::load");
            setProfile(snapshot.profile);
            setOrganization(isOrganization(snapshot.organization) ? snapshot.organization : null);
        } catch (reason) {
            setError(errorMessage(reason));
        } finally {
            setLoading(false);
        }
    }, []);

    useEffect(() => {
        void loadSnapshot();
    }, [loadSnapshot]);

    useEffect(() => {
        const onRefresh = (data: unknown) => {
            const snapshot = data as BankSnapshot | null;
            if (!snapshot?.profile) return;
            setProfile(snapshot.profile);
            setOrganization(isOrganization(snapshot.organization) ? snapshot.organization : null);
            setError("");
        };

        onPushEvent("bank::refresh", onRefresh);
        return () => offPushEvent("bank::refresh", onRefresh);
    }, []);

    const mutate = async (event: string, data: Record<string, unknown> = {}) => {
        setBusy(true);
        setError("");
        try {
            const result = await requestFromArma<BankMutation>(event, data);
            setProfile(result.profile);
            return true;
        } catch (reason) {
            setError(errorMessage(reason));
            return false;
        } finally {
            setBusy(false);
        }
    };

    const submitMovement = async (action: "deposit" | "withdraw") => {
        const amount = parsePositiveAmount(movementAmount);
        if (amount === null) {
            setError("Enter a valid amount");
            return;
        }

        await mutate(`bank::${action}`, { amount });
    };

    const submitTransfer = async () => {
        const amount = parsePositiveAmount(transferAmount);
        const target = transferTarget.trim();
        if (amount === null || target === "") {
            setError("Enter a target UID and valid amount");
            return;
        }

        if (await mutate("bank::transfer", { amount, target })) {
            setTransferTarget("");
        }
    };

    const changePin = async () => {
        if (!profile || !isValidPin(newPin) || newPin !== confirmPin) {
            return;
        }
        if (profile.pin_set && !isValidPin(currentPin)) {
            return;
        }

        if (await mutate("bank::change_pin", { currentPin, newPin })) {
            setCurrentPin("");
            setNewPin("");
            setConfirmPin("");
        }
    };

    const cashBalance = moneyValue(profile?.cash);
    const bankBalance = moneyValue(profile?.account.balance);
    const pendingEarnings = moneyValue(profile?.pending_earnings);
    const role = organization?.members.find((member) => member.uid === profile?.uid)?.role ?? "member";
    const canChangePin = Boolean(
        profile &&
        (!profile.pin_set || isValidPin(currentPin)) &&
        isValidPin(newPin) &&
        newPin === confirmPin
    );

    if (loading && !profile) {
        return (
            <div className="bank-loading-state" role="status" aria-live="polite">
                <span className="bank-loading-spinner" aria-hidden="true" />
                <strong>Loading account data</strong>
                <span>Connecting to Forge services...</span>
            </div>
        );
    }

    const applyMovementPreset = (value: number) => {
        setMovementAmount(value.toString());
    };

    const applyTransferPreset = (value: number) => {
        setTransferAmount(value.toString());
    };

    const setMaxMovement = () => {
        if (profile) {
            setMovementAmount(profile.cash);
        }
    };

    const setMaxTransfer = () => {
        if (profile) {
            setTransferAmount(profile.account.balance);
        }
    };

    return (
        <div className="content-stack bank-page">
            <section className="page-section bank-hero" aria-labelledby="page-title">
                <div className="section-copy">
                    <div className="page-header-row">
                        <div className="page-label-badge">Bank Portal</div>
                        <button
                            className="theme-toggle-btn-body"
                            type="button"
                            onClick={toggleTheme}
                            aria-label={`Switch to ${theme === "dark" ? "light" : "dark"} mode`}
                            title={`Switch to ${theme === "dark" ? "light" : "dark"} mode`}
                        >
                            {theme === "dark" ? <Sun size={15} /> : <Moon size={15} />}
                        </button>
                        <button
                            className={`refresh-btn ${loading ? "spinning" : ""}`}
                            type="button"
                            onClick={() => void loadSnapshot()}
                            disabled={loading || busy}
                            aria-label="Refresh data"
                            title="Refresh data"
                        >
                            <RefreshCw size={15} />
                        </button>
                    </div>
                    <h1 id="page-title">Player Banking</h1>
                    <p className="page-desc">
                        Securely manage your player funds, review transaction records, and coordinate with your organization.
                    </p>

                    {error && (
                        <div className="bank-status error-banner" role="alert">
                            <AlertCircle size={16} />
                            <span>{error}</span>
                        </div>
                    )}
                    {loading && (
                        <div className="bank-status loading-banner">
                            <div className="spinner-mini" />
                            <span>Loading account data...</span>
                        </div>
                    )}

                    <div className="card-container-wrapper">
                        <DigitalCard profile={profile} role={role} />
                    </div>
                </div>

                <div className="bank-summary" aria-busy={loading}>
                    <BalanceCard
                        label="Cash In Hand"
                        value={cashBalance}
                        icon={<Coins size={22} />}
                    />
                    <BalanceCard
                        label="Bank Balance"
                        value={bankBalance}
                        icon={<Landmark size={22} />}
                        featured
                    />
                    <BalanceCard
                        label="Pending Earnings"
                        value={pendingEarnings}
                        icon={<TrendingUp size={22} />}
                    />
                </div>
            </section>

            <section className="bank-workspace" aria-label="Bank workspace" aria-busy={busy}>
                <div className="bank-panel money-panel">
                    <PanelHeading
                        icon={<Coins size={18} />}
                        label="Action"
                        title="Money Movement"
                    />

                    <div className="field-group">
                        <MoneyField value={movementAmount} onInput={setMovementAmount} />
                        <div className="presets-row">
                            <button type="button" className="preset-chip" onClick={() => applyMovementPreset(100)}>$100</button>
                            <button type="button" className="preset-chip" onClick={() => applyMovementPreset(1000)}>$1K</button>
                            <button type="button" className="preset-chip" onClick={() => applyMovementPreset(10000)}>$10K</button>
                            <button type="button" className="preset-chip max-chip" onClick={setMaxMovement}>All Cash</button>
                        </div>
                    </div>

                    <div className="movement-actions">
                        <div className="split-actions">
                            <button
                                className="primary-action"
                                type="button"
                                disabled={!profile || busy}
                                onClick={() => void submitMovement("deposit")}
                            >
                                Deposit
                            </button>
                            <button
                                className="secondary-action"
                                type="button"
                                disabled={!profile || busy}
                                onClick={() => void submitMovement("withdraw")}
                            >
                                Withdraw
                            </button>
                        </div>
                        <button
                            className="secondary-action submit-earnings-action"
                            type="button"
                            disabled={!profile || busy || pendingEarnings <= 0}
                            onClick={() => void mutate("bank::submit_earnings")}
                        >
                            Claim Earnings
                        </button>
                    </div>
                </div>

                <div className="bank-panel org-panel">
                    <PanelHeading
                        icon={<Users size={18} />}
                        label="Organization"
                        title="Funds Snapshot"
                    />
                    <div className="snapshot-grid">
                        <SnapshotItem
                            label="Organization"
                            value={organization?.name ?? "Unavailable"}
                        />
                        <SnapshotItem
                            label="Your Role"
                            value={organization ? formatRole(role) : "Unavailable"}
                        />
                        <SnapshotItem
                            label="Vault Funds"
                            value={formatCurrency(moneyValue(organization?.bank))}
                            featured
                        />
                    </div>
                </div>

                <div className="bank-panel transfer-panel">
                    <PanelHeading
                        icon={<Send size={18} />}
                        label="Action"
                        title="Transfer Money"
                    />
                    <div className="field-group">
                        <label className="field">
                            <span>Target UID</span>
                            <div className="input-wrapper">
                                <input
                                    type="text"
                                    placeholder="Enter player UID..."
                                    value={transferTarget}
                                    onInput={(event) => setTransferTarget(event.currentTarget.value)}
                                />
                            </div>
                        </label>
                    </div>

                    <div className="field-group">
                        <MoneyField value={transferAmount} onInput={setTransferAmount} />
                        <div className="presets-row">
                            <button type="button" className="preset-chip" onClick={() => applyTransferPreset(100)}>$100</button>
                            <button type="button" className="preset-chip" onClick={() => applyTransferPreset(1000)}>$1K</button>
                            <button type="button" className="preset-chip" onClick={() => applyTransferPreset(10000)}>$10K</button>
                            <button type="button" className="preset-chip max-chip" onClick={setMaxTransfer}>All Funds</button>
                        </div>
                    </div>

                    <button
                        className="primary-action transfer-submit"
                        type="button"
                        disabled={!profile || busy || transferTarget.trim() === ""}
                        onClick={() => void submitTransfer()}
                    >
                        <Send size={14} style={{ marginRight: 6 }} />
                        Transfer
                    </button>
                </div>

                <div className="bank-panel pin-panel">
                    <PanelHeading
                        icon={<ShieldCheck size={18} />}
                        label="Security"
                        title={profile?.pin_set ? "Change ATM PIN" : "Set ATM PIN"}
                    />
                    <div className="pin-fields-stack">
                        {profile?.pin_set && (
                            <PinField label="Current PIN" value={currentPin} onInput={setCurrentPin} />
                        )}
                        <PinField label="New PIN" value={newPin} onInput={setNewPin} />
                        <PinField label="Confirm PIN" value={confirmPin} onInput={setConfirmPin} />
                    </div>

                    <button
                        className="secondary-action pin-submit"
                        type="button"
                        disabled={busy || !canChangePin}
                        onClick={() => void changePin()}
                    >
                        <Key size={14} style={{ marginRight: 6 }} />
                        {profile?.pin_set ? "Change PIN" : "Set PIN"}
                    </button>
                </div>

                <div className="bank-panel ledger-panel">
                    <PanelHeading
                        icon={<Info size={18} />}
                        label="Ledger"
                        title="Recent Transactions"
                    />
                    <div className="transaction-list">
                        {profile?.transactions.length ? (
                            profile.transactions
                                .slice(0, 10)
                                .map((transaction) => (
                                    <TransactionRow
                                        key={transaction.id}
                                        transaction={transaction}
                                    />
                                ))
                        ) : (
                            <div className="empty-ledger">
                                <Info size={20} />
                                <span>No transactions recorded yet</span>
                            </div>
                        )}
                    </div>
                </div>
            </section>
        </div>
    );
}

function PanelHeading({
    icon,
    label,
    title
}: {
    icon?: any;
    label: string;
    title: string;
}) {
    return (
        <div className="panel-heading">
            <div className="panel-heading-meta">
                {icon && <span className="panel-heading-icon">{icon}</span>}
                <span>{label}</span>
            </div>
            <strong>{title}</strong>
        </div>
    );
}

function MoneyField({ value, onInput }: { value: string; onInput: (value: string) => void }) {
    return (
        <label className="field">
            <span>Amount</span>
            <div className="input-wrapper currency-input">
                <span className="currency-prefix">$</span>
                <input
                    inputMode="decimal"
                    min="0.01"
                    step="0.01"
                    type="number"
                    placeholder="0.00"
                    value={value}
                    onInput={(event) => onInput(event.currentTarget.value)}
                />
            </div>
        </label>
    );
}

function PinField({
    label,
    value,
    onInput
}: {
    label: string;
    value: string;
    onInput: (value: string) => void;
}) {
    return (
        <label className="field">
            <span>{label}</span>
            <div className="input-wrapper">
                <input
                    inputMode="numeric"
                    maxLength={6}
                    type="password"
                    placeholder="******"
                    value={value}
                    onInput={(event) =>
                        onInput(event.currentTarget.value.replace(/\D/g, "").slice(0, 6))
                    }
                />
            </div>
        </label>
    );
}

function TransactionRow({ transaction }: { transaction: BankTransaction }) {
    const amount = moneyValue(transaction.amount);
    const incoming = amount >= 0;
    return (
        <div className="transaction-row">
            <div className="tx-details">
                <div className={`tx-icon ${incoming ? "in" : "out"}`} aria-hidden="true">
                    $
                </div>
                <div>
                    <strong>{transaction.description}</strong>
                    <span>{formatTimestamp(transaction.created_at)}</span>
                </div>
            </div>
            <b className={incoming ? "in" : "out"}>
                {incoming ? "+" : "-"}
                {formatCurrency(Math.abs(amount))}
            </b>
        </div>
    );
}

function BalanceCard({
    label,
    value,
    icon,
    featured = false
}: {
    label: string;
    value: number;
    icon?: any;
    featured?: boolean;
}) {
    return (
        <div className={`balance-card ${featured ? "featured" : ""}`}>
            <div className="balance-header">
                <span>{label}</span>
                {icon && <span className="balance-icon">{icon}</span>}
            </div>
            <strong>{formatCurrency(value)}</strong>
        </div>
    );
}

function SnapshotItem({
    label,
    value,
    featured = false
}: {
    label: string;
    value: string;
    featured?: boolean;
}) {
    return (
        <div className={`snapshot-item ${featured ? "featured" : ""}`}>
            <span>{label}</span>
            <strong>{value}</strong>
        </div>
    );
}

function DigitalCard({ profile, role }: { profile: BankProfile | null; role: OrgRole }) {
    const lastFour = lastFourDigits(profile?.account.id, profile?.uid);
    const formattedCardNumber = `4532 **** **** ${lastFour}`;
    const cardholder = role === "ceo" ? "CHIEF EXECUTIVE OFFICER" : "VALUED MEMBER";

    return (
        <div className="digital-card">
            <div className="card-glass-glow" />
            <div className="card-header">
                <div className="card-brand">
                    <span className="card-logo-mark">F</span>
                    <span className="card-brand-name">FORGE PLATINUM</span>
                </div>
                <span className="card-type-badge">{role === "ceo" ? "Elite Gold" : "Standard"}</span>
            </div>

            <div className="card-chip-contactless">
                <div className="card-chip">
                    <div className="chip-line horizontal" />
                    <div className="chip-line vertical" />
                </div>
                <svg
                    className="card-contactless"
                    width="22"
                    height="22"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    strokeWidth="2.5"
                    strokeLinecap="round"
                >
                    <path d="M5 17c0-2.8 2.2-5 5-5s5 2.2 5 5" />
                    <path d="M3 17c0-5 4-9 9-9s9 4 9 9" />
                    <path d="M7 17c0-1.7 1.3-3 3-3s3 1.3 3 3" />
                </svg>
            </div>

            <div className="card-number">{formattedCardNumber}</div>

            <div className="card-footer">
                <div className="card-holder">
                    <span className="card-label">CARDHOLDER</span>
                    <strong className="card-value">{cardholder}</strong>
                </div>
                <div className="card-expiry">
                    <span className="card-label">EXPIRES</span>
                    <strong className="card-value">12 / 29</strong>
                </div>
            </div>
        </div>
    );
}

function lastFourDigits(...identifiers: Array<string | undefined>) {
    for (const identifier of identifiers) {
        const digits = identifier?.replace(/\D/g, "") ?? "";
        if (digits.length >= 4) {
            return digits.slice(-4);
        }
    }

    return "0000";
}

function formatCurrency(value: number) {
    return new Intl.NumberFormat("en-US", {
        maximumFractionDigits: 2,
        style: "currency",
        currency: "USD"
    }).format(value);
}

function formatTimestamp(value: string) {
    const timestamp = new Date(value);
    return Number.isNaN(timestamp.valueOf()) ? value : timestamp.toLocaleString();
}

function formatRole(role: OrgRole) {
    return role === "ceo" ? "CEO" : "Member";
}

function parsePositiveAmount(value: string) {
    const amount = Number(value);
    return Number.isFinite(amount) && amount > 0 ? amount.toFixed(2) : null;
}

function moneyValue(value: string | undefined) {
    const amount = Number(value ?? 0);
    return Number.isFinite(amount) ? amount : 0;
}

function isValidPin(pin: string) {
    return /^\d{4,6}$/.test(pin);
}

function isOrganization(
    value: Organization | Record<string, never>
): value is Organization {
    return typeof value.name === "string" && Array.isArray(value.members);
}

function errorMessage(reason: unknown) {
    return reason instanceof Error ? reason.message : "Bank request failed";
}
