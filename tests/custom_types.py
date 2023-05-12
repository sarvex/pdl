# Copyright 2023 Google LLC
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     https://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

from dataclasses import dataclass
from typing import Tuple


@dataclass
class SizedCustomField:

    def __init__(self, value: int = 0):
        self.value = value

    def parse(self) -> Tuple['SizedCustomField', bytes]:
        return SizedCustomField(self[0]), self[1:]

    def parse_all(self) -> 'SizedCustomField':
        assert len(self) == 1
        return SizedCustomField(self[0])

    @property
    def size(self) -> int:
        return 1


@dataclass
class UnsizedCustomField:

    def __init__(self, value: int = 0):
        self.value = value

    def parse(self) -> Tuple['UnsizedCustomField', bytes]:
        return UnsizedCustomField(self[0]), self[1:]

    def parse_all(self) -> 'UnsizedCustomField':
        assert len(self) == 1
        return UnsizedCustomField(self[0])

    @property
    def size(self) -> int:
        return 1


def Checksum(span: bytes) -> int:
    return sum(span) % 256
