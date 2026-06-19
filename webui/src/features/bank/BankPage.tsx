import { useEffect, useState } from "preact/hooks";
import { sendToArma } from "../../bridge/host";

type OrgRole = "ceo" | "member";

type BankTransaction = {
    id: string;
    label: string;
    direction: "in" | "out";
    amount: number;
    timestamp: string;
};

type BankOrgSnapshot = {
    organizationName: string;
    role: OrgRole;
    orgFunds: number;
};

const transactions: BankTransaction[] = [
    {
        id: "txn-001",
        label: "Organization payday",
        direction: "in",
        amount: 1250,
        timestamp: "Today 09:14"
    },
    {
        id: "txn-002",
        label: "Vehicle service fee",
        direction: "out",
        amount: 220,
        timestamp: "Today 08:47"
    },
    {
        id: "txn-003",
        label: "Cash deposit",
        direction: "in",
        amount: 600,
        timestamp: "Yesterday 21:02"
    }
];

export function BankPage() {
    const [movementAmount, setMovementAmount] = useState("250");
    const [transferAmount, setTransferAmount] = useState("250");
    const [transferTarget, setTransferTarget] = useState("");
    const [cashBalance] = useState(2450);
    const [bankBalance, setBankBalance] = useState(18420);
    const [pendingEarnings, setPendingEarnings] = useState(3400);
    const [currentPin, setCurrentPin] = useState("");
    const [newPin, setNewPin] = useState("");
    const [confirmPin, setConfirmPin] = useState("");

    const orgSnapshot: BankOrgSnapshot = {
        organizationName: "Atlas Logistics",
        role: "member",
        orgFunds: 125000
    };

    useEffect(() => {
        sendToArma("bank::load", { source: "bank_page" });
        sendToArma("bank::load_org_snapshot", { source: "bank_page" });
        sendToArma("bank::load_earnings", { source: "bank_page" });
    }, []);

    const submitMovement = (action: "deposit" | "withdraw") => {
        const parsedAmount = Number(movementAmount);
        if (!Number.isFinite(parsedAmount) || parsedAmount <= 0) {
            return;
        }

        sendToArma(`bank::${action}`, {
            amount: Math.floor(parsedAmount)
        });
    };

    const submitTransfer = () => {
        const parsedAmount = Number(transferAmount);
        const target = transferTarget.trim();
        if (!Number.isFinite(parsedAmount) || parsedAmount <= 0 || target === "") {
            return;
        }

        sendToArma("bank::transfer", {
            amount: Math.floor(parsedAmount),
            target
        });
    };

    const submitEarnings = () => {
        if (pendingEarnings <= 0) {
            return;
        }

        sendToArma("bank::submit_earnings", {
            amount: pendingEarnings
        });

        setBankBalance((current) => current + pendingEarnings);
        setPendingEarnings(0);
    };

    const changePin = () => {
        if (!isValidPin(currentPin) || !isValidPin(newPin) || newPin !== confirmPin) {
            return;
        }

        sendToArma("bank::change_pin", {
            currentPin,
            newPin
        });

        setCurrentPin("");
        setNewPin("");
        setConfirmPin("");
    };

    const canChangePin = isValidPin(currentPin) && isValidPin(newPin) && newPin === confirmPin;

    return (
        <div className="content-stack bank-page">
            <section className="page-section bank-hero" aria-labelledby="page-title">
                <div className="section-copy">
                    <div className="page-label">Bank</div>
                    <h1 id="page-title">Player Banking</h1>
                    <p>
                        Review balances, inspect recent money movement, and send banking actions
                        through the Arma bridge.
                    </p>
                </div>

                <div className="bank-summary">
                    <BalanceCard label="Cash" value={cashBalance} />
                    <BalanceCard label="Bank" value={bankBalance} featured />
                    <BalanceCard label="Earnings" value={pendingEarnings} />
                </div>
            </section>

            <section className="bank-workspace" aria-label="Bank workspace">
                <div className="bank-panel money-panel">
                    <div className="panel-heading">
                        <span>Action</span>
                        <strong>Money Movement</strong>
                    </div>

                    <label className="field">
                        <span>Amount</span>
                        <input
                            inputMode="numeric"
                            min="1"
                            type="number"
                            value={movementAmount}
                            onInput={(event) => setMovementAmount(event.currentTarget.value)}
                        />
                    </label>

                    <div className="movement-actions">
                        <div className="split-actions">
                            <button
                                className="primary-action"
                                type="button"
                                onClick={() => submitMovement("deposit")}
                            >
                                Deposit
                            </button>
                            <button
                                className="secondary-action"
                                type="button"
                                onClick={() => submitMovement("withdraw")}
                            >
                                Withdraw
                            </button>
                        </div>

                        <button
                            className="secondary-action submit-earnings-action"
                            type="button"
                            disabled={pendingEarnings <= 0}
                            onClick={submitEarnings}
                        >
                            Submit Earnings
                        </button>
                    </div>
                </div>

                <div className="bank-panel org-panel">
                    <div className="panel-heading">
                        <span>Organization</span>
                        <strong>Funds Snapshot</strong>
                    </div>

                    <div className="snapshot-grid">
                        <SnapshotItem label="Org" value={orgSnapshot.organizationName} />
                        <SnapshotItem label="Role" value={formatRole(orgSnapshot.role)} />
                        <SnapshotItem label="Org Funds" value={formatCurrency(orgSnapshot.orgFunds)} featured />
                    </div>
                </div>

                <div className="bank-panel transfer-panel">
                    <div className="panel-heading">
                        <span>Action</span>
                        <strong>Transfer Money</strong>
                    </div>

                    <label className="field">
                        <span>Target UID</span>
                        <input
                            type="text"
                            value={transferTarget}
                            onInput={(event) => setTransferTarget(event.currentTarget.value)}
                        />
                    </label>

                    <label className="field">
                        <span>Amount</span>
                        <input
                            inputMode="numeric"
                            min="1"
                            type="number"
                            value={transferAmount}
                            onInput={(event) => setTransferAmount(event.currentTarget.value)}
                        />
                    </label>

                    <button
                        className="primary-action"
                        type="button"
                        disabled={transferTarget.trim() === ""}
                        onClick={submitTransfer}
                    >
                        Transfer
                    </button>
                </div>

                <div className="bank-panel pin-panel">
                    <div className="panel-heading">
                        <span>Security</span>
                        <strong>ATM PIN</strong>
                    </div>

                    <label className="field">
                        <span>Current PIN</span>
                        <input
                            inputMode="numeric"
                            maxLength={6}
                            type="password"
                            value={currentPin}
                            onInput={(event) => setCurrentPin(event.currentTarget.value)}
                        />
                    </label>

                    <label className="field">
                        <span>New PIN</span>
                        <input
                            inputMode="numeric"
                            maxLength={6}
                            type="password"
                            value={newPin}
                            onInput={(event) => setNewPin(event.currentTarget.value)}
                        />
                    </label>

                    <label className="field">
                        <span>Confirm PIN</span>
                        <input
                            inputMode="numeric"
                            maxLength={6}
                            type="password"
                            value={confirmPin}
                            onInput={(event) => setConfirmPin(event.currentTarget.value)}
                        />
                    </label>

                    <button
                        className="secondary-action"
                        type="button"
                        disabled={!canChangePin}
                        onClick={changePin}
                    >
                        Change PIN
                    </button>
                </div>

                <div className="bank-panel ledger-panel">
                    <div className="panel-heading">
                        <span>Ledger</span>
                        <strong>Recent Transactions</strong>
                    </div>

                    <div className="transaction-list">
                        {transactions.slice(0, 10).map((transaction) => (
                            <div key={transaction.id} className="transaction-row">
                                <div>
                                    <strong>{transaction.label}</strong>
                                    <span>{transaction.timestamp}</span>
                                </div>
                                <b className={transaction.direction}>
                                    {transaction.direction === "in" ? "+" : "-"}
                                    {formatCurrency(transaction.amount)}
                                </b>
                            </div>
                        ))}
                    </div>
                </div>
            </section>
        </div>
    );
}

function BalanceCard({ label, value, featured = false }: { label: string; value: number; featured?: boolean }) {
    return (
        <div className={`balance-card ${featured ? "featured" : ""}`}>
            <span>{label}</span>
            <strong>{formatCurrency(value)}</strong>
        </div>
    );
}

function SnapshotItem({ label, value, featured = false }: { label: string; value: string; featured?: boolean }) {
    return (
        <div className={`snapshot-item ${featured ? "featured" : ""}`}>
            <span>{label}</span>
            <strong>{value}</strong>
        </div>
    );
}

function formatCurrency(value: number) {
    return new Intl.NumberFormat("en-US", {
        maximumFractionDigits: 0,
        style: "currency",
        currency: "USD"
    }).format(value);
}

function formatRole(role: OrgRole) {
    return role === "ceo" ? "CEO" : "Member";
}

function isValidPin(pin: string) {
    return /^\d{4,6}$/.test(pin);
}
