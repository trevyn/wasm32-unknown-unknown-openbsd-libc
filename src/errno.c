int *__errno(void) {
  static int errno;
  return &errno;
}
