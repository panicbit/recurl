
bindgen \
	--ctypes-prefix '::libc' \
	--default-enum-style moduleconsts \
	/usr/include/curl/curl.h > src/raw.rs
