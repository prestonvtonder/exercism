
const CORPUS = 'abcdefghijklmnopqrstuvwxyz';
const KEY_LENGTH = 100;
const START_OFFSET = 97;

export class SimpleCipher {
  public key: string = '';

  private _shift: number[] = [];

  constructor(key?: string) {
    this.key = key || SimpleCipher.generateKey(KEY_LENGTH);
    [...this.key].map(char => this._shift.push(char.charCodeAt(0) - START_OFFSET));
  }

  static generateKey(length: number): string {
    let key = '';
    for (let i = 0; i < length; i++) {
      key += CORPUS.charAt(Math.floor(Math.random() * CORPUS.length));
    }
    return key;
  }

  encode(plaintext: string): string {
    return [...plaintext].map((char, i) => {
      const j = (char.charCodeAt(0) + this._shift[i % this._shift.length] - START_OFFSET) % CORPUS.length;
      return String.fromCharCode(j + START_OFFSET);
    }).join('');
  }

  decode(plaintext: string): string {
    return [...plaintext].map((char, i) => {
      const j = ((char.charCodeAt(0) - this._shift[((i % this._shift.length) + this._shift.length) % this._shift.length] - START_OFFSET) + CORPUS.length) % CORPUS.length;
      return String.fromCharCode(j + START_OFFSET);
    }).join('');
  }
}
