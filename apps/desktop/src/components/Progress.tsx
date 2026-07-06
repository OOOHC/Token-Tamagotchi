type ProgressProps = {
  label: string;
  value: number | null;
  max: number | null;
  prominence?: "primary" | "secondary";
};

export function Progress({ label, value, max, prominence = "secondary" }: ProgressProps) {
  const percent = value !== null && max ? Math.max(0, Math.min(100, (value / max) * 100)) : 0;
  const text =
    value !== null && max === 100
      ? `${Math.round(percent)}%`
      : value !== null && max !== null
        ? `${value}/${max}`
        : "Unknown";

  return (
    <div className={`food-meter-row food-meter-${prominence}`}>
      <div className="food-meter-header">
        <span>{label}</span>
        <span>{text}</span>
      </div>
      <div className="food-meter-track" aria-label={`${label} token food meter`}>
        <div className="food-meter-fill" style={{ width: `${percent}%` }} />
      </div>
    </div>
  );
}
