// Copyright 2020-2022 Jorge C. Leitão
// Copyright 2021 Datafuse Labs
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

/// Converts numeric type to a `String`
#[inline]
pub fn lexical_to_bytes<N: lexical_core::ToLexical>(n: N) -> Vec<u8> {
    let mut buf = Vec::<u8>::with_capacity(N::FORMATTED_SIZE_DECIMAL);
    lexical_to_bytes_mut(n, &mut buf);
    buf
}

/// Converts numeric type to a `String`
#[inline]
pub fn lexical_to_bytes_mut<N: lexical_core::ToLexical>(n: N, buf: &mut Vec<u8>) {
    buf.clear();
    buf.reserve(N::FORMATTED_SIZE_DECIMAL);
    unsafe {
        // JUSTIFICATION
        //  Benefit
        //      Allows using the faster serializer lexical core and convert to string
        //  Soundness
        //      Length of buf is set as written length afterwards. lexical_core
        //      creates a valid string, so doesn't need to be checked.
        let slice = std::slice::from_raw_parts_mut(buf.as_mut_ptr(), buf.capacity());

        //  Safety:
        //  Omits an unneeded bound check as we just ensured that we reserved `N::FORMATTED_SIZE_DECIMAL`
        #[cfg(debug_assertions)]
        {
            let len = lexical_core::write(n, slice).len();
            buf.set_len(len);
        }
        #[cfg(not(debug_assertions))]
        {
            let len = lexical_core::write(n, slice).len();
            buf.set_len(len);
        }
    }
}

/// Converts numeric type to a `String`
#[inline]
pub fn lexical_to_string<N: lexical_core::ToLexical>(n: N) -> String {
    unsafe { String::from_utf8_unchecked(lexical_to_bytes(n)) }
}
