import { useCallback, useEffect, useState } from "preact/hooks";
import { onPushEvent, offPushEvent, requestFromArma } from "../../bridge/host";

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

export function BankPage() {
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

    return (
        <div className="content-stack bank-page">
            <section className="page-section bank-hero" aria-labelledby="page-title">
                <div className="section-copy">
                    <div className="page-label">Bank</div>
                    <h1 id="page-title">Player Banking</h1>
                    <p>Review balances and manage player funds through the live Arma extension.</p>
                    {error && <div className="bank-status error" role="alert">{error}</div>}
                    {loading && <div className="bank-status">Loading account...</div>}
                </div>

                <div className="bank-summary" aria-busy={loading}>
                    <BalanceCard label="Cash" value={cashBalance} />
                    <BalanceCard label="Bank" value={bankBalance} featured />
                    <BalanceCard label="Earnings" value={pendingEarnings} />
                </div>
            </section>

            <section className="bank-workspace" aria-label="Bank workspace" aria-busy={busy}>
                <div className="bank-panel money-panel">
                    <PanelHeading label="Action" title="Money Movement" />
                    <MoneyField value={movementAmount} onInput={setMovementAmount} />
                    <div className="movement-actions">
                        <div className="split-actions">
                            <button className="primary-action" type="button" disabled={!profile || busy} onClick={() => void submitMovement("deposit")}>Deposit</button>
                            <button className="secondary-action" type="button" disabled={!profile || busy} onClick={() => void submitMovement("withdraw")}>Withdraw</button>
                        </div>
                        <button className="secondary-action submit-earnings-action" type="button" disabled={!profile || busy || pendingEarnings <= 0} onClick={() => void mutate("bank::submit_earnings")}>Submit Earnings</button>
                    </div>
                </div>

                <div className="bank-panel org-panel">
                    <PanelHeading label="Organization" title="Funds Snapshot" />
                    <div className="snapshot-grid">
                        <SnapshotItem label="Org" value={organization?.name ?? "Unavailable"} />
                        <SnapshotItem label="Role" value={organization ? formatRole(role) : "Unavailable"} />
                        <SnapshotItem label="Org Funds" value={formatCurrency(moneyValue(organization?.bank))} featured />
                    </div>
                </div>

                <div className="bank-panel transfer-panel">
                    <PanelHeading label="Action" title="Transfer Money" />
                    <label className="field">
                        <span>Target UID</span>
                        <input type="text" value={transferTarget} onInput={(event) => setTransferTarget(event.currentTarget.value)} />
                    </label>
                    <MoneyField value={transferAmount} onInput={setTransferAmount} />
                    <button className="primary-action" type="button" disabled={!profile || busy || transferTarget.trim() === ""} onClick={() => void submitTransfer()}>Transfer</button>
                </div>

                <div className="bank-panel pin-panel">
                    <PanelHeading label="Security" title={profile?.pin_set ? "Change ATM PIN" : "Set ATM PIN"} />
                    {profile?.pin_set && <PinField label="Current PIN" value={currentPin} onInput={setCurrentPin} />}
                    <PinField label="New PIN" value={newPin} onInput={setNewPin} />
                    <PinField label="Confirm PIN" value={confirmPin} onInput={setConfirmPin} />
                    <button className="secondary-action" type="button" disabled={busy || !canChangePin} onClick={() => void changePin()}>{profile?.pin_set ? "Change PIN" : "Set PIN"}</button>
                </div>

                <div className="bank-panel ledger-panel">
                    <PanelHeading label="Ledger" title="Recent Transactions" />
                    <div className="transaction-list">
                        {profile?.transactions.length ? profile.transactions.slice(0, 10).map((transaction) => <TransactionRow key={transaction.id} transaction={transaction} />) : <div className="empty-ledger">No transactions recorded</div>}
                    </div>
                </div>
            </section>
        </div>
    );
}

function PanelHeading({ label, title }: { label: string; title: string }) {
    return <div className="panel-heading"><span>{label}</span><strong>{title}</strong></div>;
}

function MoneyField({ value, onInput }: { value: string; onInput: (value: string) => void }) {
    return <label className="field"><span>Amount</span><input inputMode="decimal" min="0.01" step="0.01" type="number" value={value} onInput={(event) => onInput(event.currentTarget.value)} /></label>;
}

function PinField({ label, value, onInput }: { label: string; value: string; onInput: (value: string) => void }) {
    return <label className="field"><span>{label}</span><input inputMode="numeric" maxLength={6} type="password" value={value} onInput={(event) => onInput(event.currentTarget.value.replace(/\D/g, "").slice(0, 6))} /></label>;
}

function TransactionRow({ transaction }: { transaction: BankTransaction }) {
    const amount = moneyValue(transaction.amount);
    const incoming = amount >= 0;
    return <div className="transaction-row"><div><strong>{transaction.description}</strong><span>{formatTimestamp(transaction.created_at)}</span></div><b className={incoming ? "in" : "out"}>{incoming ? "+" : "-"}{formatCurrency(Math.abs(amount))}</b></div>;
}

function BalanceCard({ label, value, featured = false }: { label: string; value: number; featured?: boolean }) {
    return <div className={`balance-card ${featured ? "featured" : ""}`}><span>{label}</span><strong>{formatCurrency(value)}</strong></div>;
}

function SnapshotItem({ label, value, featured = false }: { label: string; value: string; featured?: boolean }) {
    return <div className={`snapshot-item ${featured ? "featured" : ""}`}><span>{label}</span><strong>{value}</strong></div>;
}

function formatCurrency(value: number) {
    return new Intl.NumberFormat("en-US", { maximumFractionDigits: 2, style: "currency", currency: "USD" }).format(value);
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

function isOrganization(value: Organization | Record<string, never>): value is Organization {
    return typeof value.name === "string" && Array.isArray(value.members);
}

function errorMessage(reason: unknown) {
    return reason instanceof Error ? reason.message : "Bank request failed";
}
