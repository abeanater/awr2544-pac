#[doc = "Register `PMPROTSET0` reader"]
pub type R = crate::R<Pmprotset0Spec>;
#[doc = "Register `PMPROTSET0` writer"]
pub type W = crate::W<Pmprotset0Spec>;
#[doc = "Field `PCS0_PROT_SET` reader - 0:0\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Pcs0ProtSetR = crate::BitReader;
#[doc = "Field `PCS0_PROT_SET` writer - 0:0\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Pcs0ProtSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCS1_PROT_SET` reader - 1:1\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Pcs1ProtSetR = crate::BitReader;
#[doc = "Field `PCS1_PROT_SET` writer - 1:1\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Pcs1ProtSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCS2_PROT_SET` reader - 2:2\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Pcs2ProtSetR = crate::BitReader;
#[doc = "Field `PCS2_PROT_SET` writer - 2:2\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Pcs2ProtSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCS3_PROT_SET` reader - 3:3\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Pcs3ProtSetR = crate::BitReader;
#[doc = "Field `PCS3_PROT_SET` writer - 3:3\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Pcs3ProtSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCS4_PROT_SET` reader - 4:4\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Pcs4ProtSetR = crate::BitReader;
#[doc = "Field `PCS4_PROT_SET` writer - 4:4\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Pcs4ProtSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCS5_PROT_SET` reader - 5:5\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Pcs5ProtSetR = crate::BitReader;
#[doc = "Field `PCS5_PROT_SET` writer - 5:5\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Pcs5ProtSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCS6_PROT_SET` reader - 6:6\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Pcs6ProtSetR = crate::BitReader;
#[doc = "Field `PCS6_PROT_SET` writer - 6:6\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Pcs6ProtSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCS7_PROT_SET` reader - 7:7\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Pcs7ProtSetR = crate::BitReader;
#[doc = "Field `PCS7_PROT_SET` writer - 7:7\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Pcs7ProtSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCS8_PROT_SET` reader - 8:8\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Pcs8ProtSetR = crate::BitReader;
#[doc = "Field `PCS8_PROT_SET` writer - 8:8\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Pcs8ProtSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCS9_PROT_SET` reader - 9:9\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Pcs9ProtSetR = crate::BitReader;
#[doc = "Field `PCS9_PROT_SET` writer - 9:9\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Pcs9ProtSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCS10_PROT_SET` reader - 10:10\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Pcs10ProtSetR = crate::BitReader;
#[doc = "Field `PCS10_PROT_SET` writer - 10:10\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Pcs10ProtSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCS11_PROT_SET` reader - 11:11\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Pcs11ProtSetR = crate::BitReader;
#[doc = "Field `PCS11_PROT_SET` writer - 11:11\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Pcs11ProtSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCS12_PROT_SET` reader - 12:12\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Pcs12ProtSetR = crate::BitReader;
#[doc = "Field `PCS12_PROT_SET` writer - 12:12\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Pcs12ProtSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCS13_PROT_SET` reader - 13:13\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Pcs13ProtSetR = crate::BitReader;
#[doc = "Field `PCS13_PROT_SET` writer - 13:13\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Pcs13ProtSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCS14_PROT_SET` reader - 14:14\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Pcs14ProtSetR = crate::BitReader;
#[doc = "Field `PCS14_PROT_SET` writer - 14:14\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Pcs14ProtSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCS15_PROT_SET` reader - 15:15\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Pcs15ProtSetR = crate::BitReader;
#[doc = "Field `PCS15_PROT_SET` writer - 15:15\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Pcs15ProtSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCS16_PROT_SET` reader - 16:16\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Pcs16ProtSetR = crate::BitReader;
#[doc = "Field `PCS16_PROT_SET` writer - 16:16\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Pcs16ProtSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCS17_PROT_SET` reader - 17:17\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Pcs17ProtSetR = crate::BitReader;
#[doc = "Field `PCS17_PROT_SET` writer - 17:17\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Pcs17ProtSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCS18_PROT_SET` reader - 18:18\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Pcs18ProtSetR = crate::BitReader;
#[doc = "Field `PCS18_PROT_SET` writer - 18:18\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Pcs18ProtSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCS19_PROT_SET` reader - 19:19\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Pcs19ProtSetR = crate::BitReader;
#[doc = "Field `PCS19_PROT_SET` writer - 19:19\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Pcs19ProtSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCS20_PROT_SET` reader - 20:20\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Pcs20ProtSetR = crate::BitReader;
#[doc = "Field `PCS20_PROT_SET` writer - 20:20\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Pcs20ProtSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCS21_PROT_SET` reader - 21:21\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Pcs21ProtSetR = crate::BitReader;
#[doc = "Field `PCS21_PROT_SET` writer - 21:21\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Pcs21ProtSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCS22_PROT_SET` reader - 22:22\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Pcs22ProtSetR = crate::BitReader;
#[doc = "Field `PCS22_PROT_SET` writer - 22:22\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Pcs22ProtSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCS23_PROT_SET` reader - 23:23\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Pcs23ProtSetR = crate::BitReader;
#[doc = "Field `PCS23_PROT_SET` writer - 23:23\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Pcs23ProtSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCS24_PROT_SET` reader - 24:24\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Pcs24ProtSetR = crate::BitReader;
#[doc = "Field `PCS24_PROT_SET` writer - 24:24\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Pcs24ProtSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCS25_PROT_SET` reader - 25:25\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Pcs25ProtSetR = crate::BitReader;
#[doc = "Field `PCS25_PROT_SET` writer - 25:25\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Pcs25ProtSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCS26_PROT_SET` reader - 26:26\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Pcs26ProtSetR = crate::BitReader;
#[doc = "Field `PCS26_PROT_SET` writer - 26:26\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Pcs26ProtSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCS27_PROT_SET` reader - 27:27\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Pcs27ProtSetR = crate::BitReader;
#[doc = "Field `PCS27_PROT_SET` writer - 27:27\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Pcs27ProtSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCS28_PROT_SET` reader - 28:28\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Pcs28ProtSetR = crate::BitReader;
#[doc = "Field `PCS28_PROT_SET` writer - 28:28\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Pcs28ProtSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCS29_PROT_SET` reader - 29:29\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Pcs29ProtSetR = crate::BitReader;
#[doc = "Field `PCS29_PROT_SET` writer - 29:29\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Pcs29ProtSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCS30_PROT_SET` reader - 30:30\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Pcs30ProtSetR = crate::BitReader;
#[doc = "Field `PCS30_PROT_SET` writer - 30:30\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Pcs30ProtSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCS31_PROT_SET` reader - 31:31\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Pcs31ProtSetR = crate::BitReader;
#[doc = "Field `PCS31_PROT_SET` writer - 31:31\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
pub type Pcs31ProtSetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    pub fn pcs0_prot_set(&self) -> Pcs0ProtSetR {
        Pcs0ProtSetR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    pub fn pcs1_prot_set(&self) -> Pcs1ProtSetR {
        Pcs1ProtSetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    pub fn pcs2_prot_set(&self) -> Pcs2ProtSetR {
        Pcs2ProtSetR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    pub fn pcs3_prot_set(&self) -> Pcs3ProtSetR {
        Pcs3ProtSetR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    pub fn pcs4_prot_set(&self) -> Pcs4ProtSetR {
        Pcs4ProtSetR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    pub fn pcs5_prot_set(&self) -> Pcs5ProtSetR {
        Pcs5ProtSetR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    pub fn pcs6_prot_set(&self) -> Pcs6ProtSetR {
        Pcs6ProtSetR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    pub fn pcs7_prot_set(&self) -> Pcs7ProtSetR {
        Pcs7ProtSetR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    pub fn pcs8_prot_set(&self) -> Pcs8ProtSetR {
        Pcs8ProtSetR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    pub fn pcs9_prot_set(&self) -> Pcs9ProtSetR {
        Pcs9ProtSetR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    pub fn pcs10_prot_set(&self) -> Pcs10ProtSetR {
        Pcs10ProtSetR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    pub fn pcs11_prot_set(&self) -> Pcs11ProtSetR {
        Pcs11ProtSetR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    pub fn pcs12_prot_set(&self) -> Pcs12ProtSetR {
        Pcs12ProtSetR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    pub fn pcs13_prot_set(&self) -> Pcs13ProtSetR {
        Pcs13ProtSetR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    pub fn pcs14_prot_set(&self) -> Pcs14ProtSetR {
        Pcs14ProtSetR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    pub fn pcs15_prot_set(&self) -> Pcs15ProtSetR {
        Pcs15ProtSetR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    pub fn pcs16_prot_set(&self) -> Pcs16ProtSetR {
        Pcs16ProtSetR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    pub fn pcs17_prot_set(&self) -> Pcs17ProtSetR {
        Pcs17ProtSetR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    pub fn pcs18_prot_set(&self) -> Pcs18ProtSetR {
        Pcs18ProtSetR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    pub fn pcs19_prot_set(&self) -> Pcs19ProtSetR {
        Pcs19ProtSetR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    pub fn pcs20_prot_set(&self) -> Pcs20ProtSetR {
        Pcs20ProtSetR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    pub fn pcs21_prot_set(&self) -> Pcs21ProtSetR {
        Pcs21ProtSetR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    pub fn pcs22_prot_set(&self) -> Pcs22ProtSetR {
        Pcs22ProtSetR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    pub fn pcs23_prot_set(&self) -> Pcs23ProtSetR {
        Pcs23ProtSetR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    pub fn pcs24_prot_set(&self) -> Pcs24ProtSetR {
        Pcs24ProtSetR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    pub fn pcs25_prot_set(&self) -> Pcs25ProtSetR {
        Pcs25ProtSetR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    pub fn pcs26_prot_set(&self) -> Pcs26ProtSetR {
        Pcs26ProtSetR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    pub fn pcs27_prot_set(&self) -> Pcs27ProtSetR {
        Pcs27ProtSetR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    pub fn pcs28_prot_set(&self) -> Pcs28ProtSetR {
        Pcs28ProtSetR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    pub fn pcs29_prot_set(&self) -> Pcs29ProtSetR {
        Pcs29ProtSetR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    pub fn pcs30_prot_set(&self) -> Pcs30ProtSetR {
        Pcs30ProtSetR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    pub fn pcs31_prot_set(&self) -> Pcs31ProtSetR {
        Pcs31ProtSetR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    #[must_use]
    pub fn pcs0_prot_set(&mut self) -> Pcs0ProtSetW<Pmprotset0Spec> {
        Pcs0ProtSetW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    #[must_use]
    pub fn pcs1_prot_set(&mut self) -> Pcs1ProtSetW<Pmprotset0Spec> {
        Pcs1ProtSetW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    #[must_use]
    pub fn pcs2_prot_set(&mut self) -> Pcs2ProtSetW<Pmprotset0Spec> {
        Pcs2ProtSetW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    #[must_use]
    pub fn pcs3_prot_set(&mut self) -> Pcs3ProtSetW<Pmprotset0Spec> {
        Pcs3ProtSetW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    #[must_use]
    pub fn pcs4_prot_set(&mut self) -> Pcs4ProtSetW<Pmprotset0Spec> {
        Pcs4ProtSetW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    #[must_use]
    pub fn pcs5_prot_set(&mut self) -> Pcs5ProtSetW<Pmprotset0Spec> {
        Pcs5ProtSetW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    #[must_use]
    pub fn pcs6_prot_set(&mut self) -> Pcs6ProtSetW<Pmprotset0Spec> {
        Pcs6ProtSetW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    #[must_use]
    pub fn pcs7_prot_set(&mut self) -> Pcs7ProtSetW<Pmprotset0Spec> {
        Pcs7ProtSetW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    #[must_use]
    pub fn pcs8_prot_set(&mut self) -> Pcs8ProtSetW<Pmprotset0Spec> {
        Pcs8ProtSetW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    #[must_use]
    pub fn pcs9_prot_set(&mut self) -> Pcs9ProtSetW<Pmprotset0Spec> {
        Pcs9ProtSetW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    #[must_use]
    pub fn pcs10_prot_set(&mut self) -> Pcs10ProtSetW<Pmprotset0Spec> {
        Pcs10ProtSetW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    #[must_use]
    pub fn pcs11_prot_set(&mut self) -> Pcs11ProtSetW<Pmprotset0Spec> {
        Pcs11ProtSetW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    #[must_use]
    pub fn pcs12_prot_set(&mut self) -> Pcs12ProtSetW<Pmprotset0Spec> {
        Pcs12ProtSetW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    #[must_use]
    pub fn pcs13_prot_set(&mut self) -> Pcs13ProtSetW<Pmprotset0Spec> {
        Pcs13ProtSetW::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    #[must_use]
    pub fn pcs14_prot_set(&mut self) -> Pcs14ProtSetW<Pmprotset0Spec> {
        Pcs14ProtSetW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    #[must_use]
    pub fn pcs15_prot_set(&mut self) -> Pcs15ProtSetW<Pmprotset0Spec> {
        Pcs15ProtSetW::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    #[must_use]
    pub fn pcs16_prot_set(&mut self) -> Pcs16ProtSetW<Pmprotset0Spec> {
        Pcs16ProtSetW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    #[must_use]
    pub fn pcs17_prot_set(&mut self) -> Pcs17ProtSetW<Pmprotset0Spec> {
        Pcs17ProtSetW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    #[must_use]
    pub fn pcs18_prot_set(&mut self) -> Pcs18ProtSetW<Pmprotset0Spec> {
        Pcs18ProtSetW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    #[must_use]
    pub fn pcs19_prot_set(&mut self) -> Pcs19ProtSetW<Pmprotset0Spec> {
        Pcs19ProtSetW::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    #[must_use]
    pub fn pcs20_prot_set(&mut self) -> Pcs20ProtSetW<Pmprotset0Spec> {
        Pcs20ProtSetW::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    #[must_use]
    pub fn pcs21_prot_set(&mut self) -> Pcs21ProtSetW<Pmprotset0Spec> {
        Pcs21ProtSetW::new(self, 21)
    }
    #[doc = "Bit 22 - 22:22\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    #[must_use]
    pub fn pcs22_prot_set(&mut self) -> Pcs22ProtSetW<Pmprotset0Spec> {
        Pcs22ProtSetW::new(self, 22)
    }
    #[doc = "Bit 23 - 23:23\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    #[must_use]
    pub fn pcs23_prot_set(&mut self) -> Pcs23ProtSetW<Pmprotset0Spec> {
        Pcs23ProtSetW::new(self, 23)
    }
    #[doc = "Bit 24 - 24:24\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    #[must_use]
    pub fn pcs24_prot_set(&mut self) -> Pcs24ProtSetW<Pmprotset0Spec> {
        Pcs24ProtSetW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    #[must_use]
    pub fn pcs25_prot_set(&mut self) -> Pcs25ProtSetW<Pmprotset0Spec> {
        Pcs25ProtSetW::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    #[must_use]
    pub fn pcs26_prot_set(&mut self) -> Pcs26ProtSetW<Pmprotset0Spec> {
        Pcs26ProtSetW::new(self, 26)
    }
    #[doc = "Bit 27 - 27:27\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    #[must_use]
    pub fn pcs27_prot_set(&mut self) -> Pcs27ProtSetW<Pmprotset0Spec> {
        Pcs27ProtSetW::new(self, 27)
    }
    #[doc = "Bit 28 - 28:28\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    #[must_use]
    pub fn pcs28_prot_set(&mut self) -> Pcs28ProtSetW<Pmprotset0Spec> {
        Pcs28ProtSetW::new(self, 28)
    }
    #[doc = "Bit 29 - 29:29\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    #[must_use]
    pub fn pcs29_prot_set(&mut self) -> Pcs29ProtSetW<Pmprotset0Spec> {
        Pcs29ProtSetW::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    #[must_use]
    pub fn pcs30_prot_set(&mut self) -> Pcs30ProtSetW<Pmprotset0Spec> {
        Pcs30ProtSetW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Sets the corresponding bit in PMPROTSET0 and PMPROTCLR0 registers 0 = Has no effect Only those bits which have a slave at the corresponding bit position are implemented. Hence, the size of this register is device dependent. Writes to unimplemented bits have no effect and reads yield 0."]
    #[inline(always)]
    #[must_use]
    pub fn pcs31_prot_set(&mut self) -> Pcs31ProtSetW<Pmprotset0Spec> {
        Pcs31ProtSetW::new(self, 31)
    }
}
#[doc = "Set-only register to protect PCS frames 0 to 31\n\nYou can [`read`](crate::Reg::read) this register and get [`pmprotset0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmprotset0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pmprotset0Spec;
impl crate::RegisterSpec for Pmprotset0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmprotset0::R`](R) reader structure"]
impl crate::Readable for Pmprotset0Spec {}
#[doc = "`write(|w| ..)` method takes [`pmprotset0::W`](W) writer structure"]
impl crate::Writable for Pmprotset0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMPROTSET0 to value 0"]
impl crate::Resettable for Pmprotset0Spec {
    const RESET_VALUE: u32 = 0;
}
