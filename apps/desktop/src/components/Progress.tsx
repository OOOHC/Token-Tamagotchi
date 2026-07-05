type ProgressProps = {
  label: string;
  value: number | null;
  max: number | null;
};

export function Progress({ label, value, max }: ProgressProps) {
  const percent = value !== null && max ? Math.max(0, Math.min(100, (value / max) * 100)) : 0;
  const text = value !== null && max !== null ? `${value}/${max}` : "Unknown";

  return (
    <div className="progress-row">
      <div className="progress-header">
        <span>{label}</span>
        <span>{text}</span>
      </div>
      <div className="progress-track">
        <div className="progress-fill" style={{ width: `${percent}%` }} />
      </div>
    </div>
  );
}

