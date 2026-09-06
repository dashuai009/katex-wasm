export function median(values) {
  if (!values.length) return null;
  const sorted = [...values].sort((a, b) => a - b),
    middle = Math.floor(sorted.length / 2);
  return sorted.length % 2
    ? sorted[middle]
    : (sorted[middle - 1] + sorted[middle]) / 2;
}
export function stats(values) {
  if (!values.length)
    return { count: 0, total: null, mean: null, median: null, p95: null };
  const sorted = [...values].sort((a, b) => a - b),
    total = values.reduce((a, b) => a + b, 0);
  return {
    count: values.length,
    total,
    mean: total / values.length,
    median: median(sorted),
    p95: sorted[Math.ceil(sorted.length * 0.95) - 1],
  };
}
