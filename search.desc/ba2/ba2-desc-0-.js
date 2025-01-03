searchState.loadedDescShard("ba2", 0, "Archives come in various flavors, and the specific variant …\nA wrapper for <code>&amp;[u8]</code> that provides convenient string …\nA wrapper for <code>Vec&lt;u8&gt;</code> that provides convenient string …\nMakes a shallow copy of the input.\nA trait that extends <code>&amp;[u8]</code> with string oriented methods.\nA trait that extends <code>Vec&lt;u8&gt;</code> with string oriented methods.\nA trait that creates an optionally compressed container …\nThe data will finish in a compressed state.\nIndicates whether the operation should finish by …\nMakes a deep copy of the input.\nThe data will finish in a decompressed state.\nThe file format for a given archive.\nA trait that enables reading from various sources.\nA trait that enables reading from various sources, with …\nReturn this byte slice as a <code>&amp;BStr</code>.\nReturn this byte slice as a <code>&amp;mut BStr</code>.\nReturns an iterator over the bytes in this byte string.\nReturns an iterator over the Unicode scalar values in this …\nReturns an iterator over the Unicode scalar values in this …\nReturns true if and only if this byte string contains the …\nCreates a draining iterator that removes the specified …\nReturns true if and only if this byte string has the given …\nEscapes this byte string into a sequence of <code>char</code> values.\nReturns an iterator over the fields in a byte string, …\nReturns an iterator over the fields in a byte string, …\nReturns the index of the first occurrence of the given …\nReturns the index of the first occurrence of the given …\nReturns the index of the first occurrence of any of the …\nReturns the index of the first occurrence of the given …\nReturns an iterator of the non-overlapping occurrences of …\nReturns the index of the first non-ASCII byte in this byte …\nReturns the index of the first occurrence of a byte that …\nFallout 4\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nMakes a compressed instance of <code>Self</code> using the given data.\nMakes a decompressed instance of <code>Self</code> using the given data.\nCreate an immutable byte string from an OS string slice.\nLossily create a new byte string from an OS string slice.\nCreate a new byte string from an owned OS string.\nCreate an immutable byte string from a file path.\nCreate a new byte string from an owned file path.\nLossily create a new byte string from a file path.\nCreate a new owned byte string from the given byte slice.\nReturns an iterator over the grapheme clusters in this …\nReturns an iterator over the grapheme clusters in this …\nGuesses the archive format for a given source.\nInserts the given codepoint into this <code>Vec&lt;u8&gt;</code> at a …\nInserts the given byte string into this byte string at a …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nConverts this byte string into an OS string, in place.\nLossily converts this byte string into an OS string, in …\nConverts this byte string into an owned file path, in …\nLossily converts this byte string into an owned file path, …\nConverts a <code>Vec&lt;u8&gt;</code> into a <code>String</code> if and only if this byte …\nLossily converts a <code>Vec&lt;u8&gt;</code> into a <code>String</code>. If this byte …\nUnsafely convert this byte string into a <code>String</code>, without …\nReturns true if and only if every byte in this byte string …\nReturns true if and only if the entire byte string is …\nReturns the last byte in this byte string, if it’s …\nAn iterator over all lines in a byte string, without their …\nAn iterator over all lines in a byte string, including …\nConvert this byte string to its lowercase ASCII equivalent …\nConvert this byte string to its uppercase ASCII equivalent …\nDirectly creates a <code>BStr</code> slice from anything that can be …\nConstructs a new <code>BString</code> from the given <code>Vec</code>.\nRemoves the last byte from this <code>Vec&lt;u8&gt;</code> and returns it.\nRemoves the last codepoint from this <code>Vec&lt;u8&gt;</code> and returns …\nConvenience using statements for traits that are needed to …\nAppends the given byte to the end of this byte string.\nAppends the given <code>char</code> to the end of this byte string.\nAppends the given slice to the end of this byte string. …\nReads an instance of <code>Self::Item</code> from the given source.\nReads an instance of <code>Self::Item</code> from the given source, …\nRemoves a <code>char</code> from this <code>Vec&lt;u8&gt;</code> at the given byte …\nCreate a new byte string by repeating this byte string <code>n</code> …\nReplace all matches of the given needle with the given …\nReplace all matches of the given needle with the given …\nRemoves the specified range in this byte string and …\nReplace up to <code>limit</code> matches of the given needle with the …\nReplace up to <code>limit</code> matches of the given needle with the …\nReverse the bytes in this string, in place.\nReverse the codepoints in this string, in place.\nReverse the graphemes in this string, in place.\nReturns the index of the last occurrence of the given …\nReturns the index of the last occurrence of the given …\nReturns the index of the last occurrence of any of the …\nReturns the index of the last occurrence of the given …\nReturns an iterator of the non-overlapping occurrences of …\nReturns the index of the last occurrence of a byte that is …\nSplit this byte string at the last occurrence of <code>splitter</code>.\nReturns an iterator over substrings of this byte string, …\nReturns an iterator of at most <code>limit</code> substrings of this …\nReturns an iterator over the sentences in this byte string …\nReturns an iterator over the sentences in this byte string.\nSplit this byte string at the first occurrence of <code>splitter</code>.\nReturns an iterator over substrings of this byte string, …\nReturns an iterator of at most <code>limit</code> substrings of this …\nReturns true if and only if this byte string has the given …\nTES III: Morrowind\nTES IV: Oblivion\nReturns a new <code>Vec&lt;u8&gt;</code> containing the ASCII lowercase …\nReturns a new <code>Vec&lt;u8&gt;</code> containing the ASCII uppercase …\nReturns a new <code>Vec&lt;u8&gt;</code> containing the lowercase equivalent …\nWrites the lowercase equivalent of this byte string into …\nCreate an OS string slice from this byte string.\nLossily create an OS string slice from this byte string.\nCreate a path slice from this byte string.\nLossily create a path slice from this byte string.\nSafely convert this byte string into a <code>&amp;str</code> if it’s …\nConvert this byte string to a valid UTF-8 string by …\nCopy the contents of this byte string into the given owned …\nUnsafely convert this byte string into a <code>&amp;str</code>, without …\nReturns a new <code>Vec&lt;u8&gt;</code> containing the uppercase equivalent …\nWrites the uppercase equivalent of this byte string into …\nReturn a byte string slice with leading and trailing …\nReturn a byte string slice with trailing whitespace …\nReturn a byte string slice with trailing characters …\nReturn a byte string slice with leading whitespace removed.\nReturn a byte string slice with leading characters …\nReturn a byte string slice with leading and trailing …\nUnescapes the given string into its raw bytes.\nIterate over chunks of valid UTF-8.\nReturns an iterator over the words in this byte string …\nReturns an iterator over the words in this byte string. If …\nReturns an iterator over the words and their byte offsets …\nReturns an iterator over the words in this byte string, …\nRepresents the FO4 revision of the ba2 format.\nA key for indexing into the relevant mapping.\nInfo about the contents of the given archive.\nSee also <code>ArchiveOptions</code>.\nRepresents a chunk of a file within the FO4 virtual …\nCommon parameters to configure how chunks are compressed.\nSee also <code>ChunkCompressionOptions</code>.\nA list of all compression methods supported by the ba2 …\nSpecifies the compression level to use when compressing …\nA DX10 archive can only contain .dds files (Microsoft …\nFile header for DX10 archives.\nContains the error value\nFallout 4.\nFallout 4 on the xbox.\nFallout 76.\nRepresents a file within the FO4 virtual filesystem.\nFile is at chunk capacity.\nSee also <code>Hash</code>.\nOptionally present file header.\nCommon parameters to configure how files are read.\nSee also <code>FileReadOptions</code>.\nCommon parameters to configure how files are written.\nSee also <code>FileWriteOptions</code>.\nRepresents the file format for an archive.\nA GNMF archive can only contain .gnf files (Sony GNM).\nFile header for GNMF archives.\nA GNRL archive can contain any kind of file.\nThe underlying hash object used to uniquely identify …\nA more specialized format leveraging lz4’s fast …\nContains the success value\nStarfield.\nIndicates the version of an archive.\nThe default compression format, compatible with all games …\nThe file’s parent path crc.\nPanics\nThe first 4 bytes of the file’s extension.\nThe file’s stem crc.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nProduces a hash using the given path.\nProduces a hash using the given path.\nPanics\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nSee here for more info.\nThis setting corresponds to the <em>Single Mip Chunk Area</em> …\nSee also <code>mip_chunk_height</code>.\nPanics\nPanics\nPanics\nPanics\nPanics\nInitial format introduced in Fallout 4.\nIntoduced in Starfield.\nIntoduced in Starfield.\nIntoduced in the Fallout 4 next-gen update.\nIntoduced in the Fallout 4 next-gen update.\nRepresents the TES3 revision of the bsa format.\nA key for indexing into the relevant mapping.\nContains the error value\nRepresents a file within the TES3 virtual filesystem.\nSee also <code>Hash</code>.\nThe underlying hash object used to uniquely identify …\nContains the success value\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nProduces a hash using the given path.\nProduces a hash using the given path.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nRepresents the TES4 revision of the bsa format.\nArchive flags can impact the layout of an archive, or how …\nA key for indexing into the relevant mapping.\nSee also <code>ArchiveOptions</code>.\nSpecifies file types contained within an archive.\nCompresses the data within the archive.\nSpecifies the codec to use when performing …\nIncludes directory paths within the archive.\nRepresents a directory within the TES4 virtual filesystem.\nSee also <code>Hash</code>.\nA key for indexing into the relevant mapping.\nWrites the full (virtual) path of a file next to the data …\nContains the error value\nIncludes filenames within the archive.\nFallout: New Vegas.\nFallout 3.\nRepresents a file within the TES4 virtual filesystem.\nCommon parameters to configure how files are …\nSee also <code>FileCompressionOptions</code>.\nSee also <code>Hash</code>.\nCommon parameters to configure how files are read.\nSee also <code>FileReadOptions</code>.\nThe underlying hash object used to uniquely identify …\nThe default compression codec.\nContains the success value\nImpacts runtime parsing.\nImpacts runtime parsing.\nImpacts runtime parsing.\nImpacts runtime parsing.\nThe Elder Scrolls V: Skyrim - Special Edition.\nThe Elder Scrolls IV: Oblivion.\nThe Elder Scrolls V: Skyrim.\nThe archive version.\nWrites the archive in the xbox (big-endian) format.\nUses the xmem codec from XNA 4.0 to compress the archive.\nGet a flags value with all known bits set.\nGet a flags value with all known bits set.\nThe bitwise and (<code>&amp;</code>) of the bits in two flags values.\nThe bitwise and (<code>&amp;</code>) of the bits in two flags values.\nThe bitwise and (<code>&amp;</code>) of the bits in two flags values.\nThe bitwise and (<code>&amp;</code>) of the bits in two flags values.\nThe bitwise or (<code>|</code>) of the bits in two flags values.\nThe bitwise or (<code>|</code>) of the bits in two flags values.\nThe bitwise or (<code>|</code>) of the bits in two flags values.\nThe bitwise or (<code>|</code>) of the bits in two flags values.\nGet the underlying bits value.\nGet the underlying bits value.\nThe bitwise exclusive-or (<code>^</code>) of the bits in two flags …\nThe bitwise exclusive-or (<code>^</code>) of the bits in two flags …\nThe bitwise exclusive-or (<code>^</code>) of the bits in two flags …\nThe bitwise exclusive-or (<code>^</code>) of the bits in two flags …\nThe bitwise negation (<code>!</code>) of the bits in a flags value, …\nThe bitwise negation (<code>!</code>) of the bits in a flags value, …\nWhether all set bits in a source flags value are also set …\nWhether all set bits in a source flags value are also set …\nThe intersection of a source flags value with the …\nThe intersection of a source flags value with the …\nGet a flags value with all bits unset.\nGet a flags value with all bits unset.\nThe bitwise or (<code>|</code>) of the bits in each flags value.\nThe bitwise or (<code>|</code>) of the bits in each flags value.\nThe first character of the path (directory) or stem (file).\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nConvert from a bits value.\nConvert from a bits value.\nConvert from a bits value exactly.\nConvert from a bits value exactly.\nConvert from a bits value, unsetting any unknown bits.\nConvert from a bits value, unsetting any unknown bits.\nThe bitwise or (<code>|</code>) of the bits in each flags value.\nThe bitwise or (<code>|</code>) of the bits in each flags value.\nGet a flags value with the bits of a flag with the given …\nGet a flags value with the bits of a flag with the given …\nProduces a hash using the given path.\nProduces a hash using the given path.\nProduces a hash using the given path.\nProduces a hash using the given path.\nThe bitwise or (<code>|</code>) of the bits in two flags values.\nThe bitwise or (<code>|</code>) of the bits in two flags values.\nThe bitwise and (<code>&amp;</code>) of the bits in two flags values.\nThe bitwise and (<code>&amp;</code>) of the bits in two flags values.\nWhether any set bits in a source flags value are also set …\nWhether any set bits in a source flags value are also set …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nWhether all known bits in this flags value are set.\nWhether all known bits in this flags value are set.\nWhether all bits in this flags value are unset.\nWhether all bits in this flags value are unset.\nYield a set of contained flags values.\nYield a set of contained flags values.\nYield a set of contained named flags values.\nYield a set of contained named flags values.\nThe last character of the path (directory) or stem (file).\nThe second to last character of the path (directory) or …\nThe length of the path (directory) or stem (file).\nThe bitwise negation (<code>!</code>) of the bits in a flags value, …\nThe bitwise negation (<code>!</code>) of the bits in a flags value, …\nThe intersection of a source flags value with the …\nThe intersection of a source flags value with the …\nCall <code>insert</code> when <code>value</code> is <code>true</code> or <code>remove</code> when <code>value</code> is …\nCall <code>insert</code> when <code>value</code> is <code>true</code> or <code>remove</code> when <code>value</code> is …\nThe intersection of a source flags value with the …\nThe intersection of a source flags value with the …\nThe intersection of a source flags value with the …\nThe intersection of a source flags value with the …\nThe bitwise exclusive-or (<code>^</code>) of the bits in two flags …\nThe bitwise exclusive-or (<code>^</code>) of the bits in two flags …\nThe bitwise exclusive-or (<code>^</code>) of the bits in two flags …\nThe bitwise exclusive-or (<code>^</code>) of the bits in two flags …\nThe bitwise or (<code>|</code>) of the bits in two flags values.\nThe bitwise or (<code>|</code>) of the bits in two flags values.")