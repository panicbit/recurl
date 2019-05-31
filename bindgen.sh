
bindgen \
	--ctypes-prefix '::libc' \
	--constified-enum-module '.*' \
	/usr/include/curl/curl.h > src/raw.rs
