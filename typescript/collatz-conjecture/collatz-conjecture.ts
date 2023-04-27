export function steps(count: number): number {
  if (count < 1) throw new Error('Only positive numbers are allowed')
  let counter = 0;
  while (count !== 1) {
    if (count % 2 === 0) count /= 2;
    else count = 3 * count + 1;
    counter++;
  }
  return counter;
}

