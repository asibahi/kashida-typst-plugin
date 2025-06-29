#let k = plugin("kashida.wasm")

/// Add kashidas to Arabic string
#let kashida(
  /// Arabic text. It is unchecked if this is valid Arabic.
  text,
  /// Kashida count
  count,
) = {
  str(k.add_kashida(
    bytes(text),
    count.to-bytes(size: 4)
  ))
}

// usage
#kashida(
  "بسم الله الرحمن الرحيم",
  19
)