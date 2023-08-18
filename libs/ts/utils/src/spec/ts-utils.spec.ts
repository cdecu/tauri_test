import { calculateFlow, calculatePressure } from '../lib/permeability';

describe('calculatePressure', () => {
  it('should work', () => {
    const p = calculatePressure(1);
    const f = calculateFlow(p);
    expect(f).toEqual(1.0);
  });
});
describe('calculateFlow', () => {
  it('should work', () => {
    const f = calculateFlow(1);
    const p = calculatePressure(f);
    expect(p).toEqual(1.0);
  });
});
