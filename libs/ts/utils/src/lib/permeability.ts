export function calculatePressure(flowrate: number) {
  return 2 * flowrate;
}

export function calculateFlowRate(pressure: number) {
  return pressure / 2;
}
