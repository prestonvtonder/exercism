export class Matrix {
  private _rows: number[][] = [];
  private _cols: number[][] = [];

  constructor(raw: string) {
    this.process(raw);
  }

  private process(raw: string): void {
    this._rows = raw
      .split('\n')
      .map(row => row.split(' ').map(number => +number))
    this._cols = this.transpose(this._rows);
  }

  private transpose(matrix: number[][]): number[][] {
    return matrix[0].map((_, i) => matrix.map(row => row[i]));
  }

  get rows(): number[][] {
    return this._rows;
  }

  get columns(): number[][] {
    return this._cols;
  }
}
