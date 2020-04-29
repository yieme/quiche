// Copyright (C) 2020, Cloudflare, Inc.
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are
// met:
//
//     * Redistributions of source code must retain the above copyright notice,
//       this list of conditions and the following disclaimer.
//
//     * Redistributions in binary form must reproduce the above copyright
//       notice, this list of conditions and the following disclaimer in the
//       documentation and/or other materials provided with the distribution.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS
// IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO,
// THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
// PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR
// CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
// EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
// PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
// PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
// LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
// NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
// SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

/// An Extensible Priority.
///
/// This holds the extensible priority parameters, with methods for
/// serialization, deserialization and conversion to quiche's stream priority
/// space.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Priority {
    /// Urgency.
    pub urgency: u8,
    /// Incremental.
    pub incremental: bool,
}

impl Default for Priority {
    fn default() -> Self {
        Priority {
            urgency: 1,
            incremental: false,
        }
    }
}

impl Priority {
    /// Converts from the priority wire format.
    pub fn from_wire(priority_field: &str) -> Self {
        let mut priority = Priority::default();

        for param in priority_field.split(',') {
            if param.trim() == "i" {
                priority.incremental = true;
            }

            if param.trim().starts_with("u=") {
                // u is an sh-integer (an i64) but it has a constrained range of
                // 0-7. So detect anything outside that range and clamp it to
                // the lowest priority in order to avoid it interfering with
                // valid items.
                //
                // TODO: this also detects when u is not an
                // sh-integer and clamps it in the same way. A real structured
                // header parser would actually fail to parse.
                let mut u =
                    i64::from_str_radix(param.rsplit('=').next().unwrap(), 10)
                        .unwrap_or(7);

                if u < 0 || u > 7 {
                    u = 7
                };

                priority.urgency = u as u8;
            }
        }

        priority
    }

    /// Converts to the priority wire format.
    pub fn to_wire(self) -> String {
        let mut response_priority = format!("u={}", self.urgency);
        if self.incremental {
            response_priority.push_str(",i");
        }

        response_priority
    }
}