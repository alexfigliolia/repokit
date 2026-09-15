export class FileParseError {
  constructor(
    public readonly path: string,
    public readonly message?: string,
  ) {}
}
