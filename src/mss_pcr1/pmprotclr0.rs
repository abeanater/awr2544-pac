#[doc = "Register `PMPROTCLR0` reader"]
pub type R = crate::R<Pmprotclr0Spec>;
#[doc = "Register `PMPROTCLR0` writer"]
pub type W = crate::W<Pmprotclr0Spec>;
#[doc = "Field `PCS0_PROT_CLR` reader - 0:0\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
pub type Pcs0ProtClrR = crate::BitReader;
#[doc = "Field `PCS0_PROT_CLR` writer - 0:0\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
pub type Pcs0ProtClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCS1_PROT_CLR` reader - 1:1\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
pub type Pcs1ProtClrR = crate::BitReader;
#[doc = "Field `PCS1_PROT_CLR` writer - 1:1\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
pub type Pcs1ProtClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCS2_PROT_CLR` reader - 2:2\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
pub type Pcs2ProtClrR = crate::BitReader;
#[doc = "Field `PCS2_PROT_CLR` writer - 2:2\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
pub type Pcs2ProtClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCS3_PROT_CLR` reader - 3:3\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
pub type Pcs3ProtClrR = crate::BitReader;
#[doc = "Field `PCS3_PROT_CLR` writer - 3:3\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
pub type Pcs3ProtClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCS4_PROT_CLR` reader - 4:4\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
pub type Pcs4ProtClrR = crate::BitReader;
#[doc = "Field `PCS4_PROT_CLR` writer - 4:4\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
pub type Pcs4ProtClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCS5_PROT_CLR` reader - 5:5\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
pub type Pcs5ProtClrR = crate::BitReader;
#[doc = "Field `PCS5_PROT_CLR` writer - 5:5\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
pub type Pcs5ProtClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCS6_PROT_CLR` reader - 6:6\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
pub type Pcs6ProtClrR = crate::BitReader;
#[doc = "Field `PCS6_PROT_CLR` writer - 6:6\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
pub type Pcs6ProtClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCS7_PROT_CLR` reader - 7:7\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
pub type Pcs7ProtClrR = crate::BitReader;
#[doc = "Field `PCS7_PROT_CLR` writer - 7:7\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
pub type Pcs7ProtClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCS8_PROT_CLR` reader - 8:8\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
pub type Pcs8ProtClrR = crate::BitReader;
#[doc = "Field `PCS8_PROT_CLR` writer - 8:8\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
pub type Pcs8ProtClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCS9_PROT_CLR` reader - 9:9\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
pub type Pcs9ProtClrR = crate::BitReader;
#[doc = "Field `PCS9_PROT_CLR` writer - 9:9\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
pub type Pcs9ProtClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCS10_PROT_CLR` reader - 10:10\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
pub type Pcs10ProtClrR = crate::BitReader;
#[doc = "Field `PCS10_PROT_CLR` writer - 10:10\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
pub type Pcs10ProtClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCS11_PROT_CLR` reader - 11:11\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
pub type Pcs11ProtClrR = crate::BitReader;
#[doc = "Field `PCS11_PROT_CLR` writer - 11:11\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
pub type Pcs11ProtClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCS12_PROT_CLR` reader - 12:12\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
pub type Pcs12ProtClrR = crate::BitReader;
#[doc = "Field `PCS12_PROT_CLR` writer - 12:12\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
pub type Pcs12ProtClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCS13_PROT_CLR` reader - 13:13\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
pub type Pcs13ProtClrR = crate::BitReader;
#[doc = "Field `PCS13_PROT_CLR` writer - 13:13\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
pub type Pcs13ProtClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCS14_PROT_CLR` reader - 14:14\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
pub type Pcs14ProtClrR = crate::BitReader;
#[doc = "Field `PCS14_PROT_CLR` writer - 14:14\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
pub type Pcs14ProtClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCS15_PROT_CLR` reader - 15:15\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
pub type Pcs15ProtClrR = crate::BitReader;
#[doc = "Field `PCS15_PROT_CLR` writer - 15:15\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
pub type Pcs15ProtClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCS16_PROT_CLR` reader - 16:16\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
pub type Pcs16ProtClrR = crate::BitReader;
#[doc = "Field `PCS16_PROT_CLR` writer - 16:16\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
pub type Pcs16ProtClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCS17_PROT_CLR` reader - 17:17\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
pub type Pcs17ProtClrR = crate::BitReader;
#[doc = "Field `PCS17_PROT_CLR` writer - 17:17\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
pub type Pcs17ProtClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCS18_PROT_CLR` reader - 18:18\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
pub type Pcs18ProtClrR = crate::BitReader;
#[doc = "Field `PCS18_PROT_CLR` writer - 18:18\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
pub type Pcs18ProtClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCS19_PROT_CLR` reader - 19:19\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
pub type Pcs19ProtClrR = crate::BitReader;
#[doc = "Field `PCS19_PROT_CLR` writer - 19:19\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
pub type Pcs19ProtClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCS20_PROT_CLR` reader - 20:20\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
pub type Pcs20ProtClrR = crate::BitReader;
#[doc = "Field `PCS20_PROT_CLR` writer - 20:20\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
pub type Pcs20ProtClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCS21_PROT_CLR` reader - 21:21\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
pub type Pcs21ProtClrR = crate::BitReader;
#[doc = "Field `PCS21_PROT_CLR` writer - 21:21\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
pub type Pcs21ProtClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCS22_PROT_CLR` reader - 22:22\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
pub type Pcs22ProtClrR = crate::BitReader;
#[doc = "Field `PCS22_PROT_CLR` writer - 22:22\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
pub type Pcs22ProtClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCS23_PROT_CLR` reader - 23:23\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
pub type Pcs23ProtClrR = crate::BitReader;
#[doc = "Field `PCS23_PROT_CLR` writer - 23:23\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
pub type Pcs23ProtClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCS24_PROT_CLR` reader - 24:24\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
pub type Pcs24ProtClrR = crate::BitReader;
#[doc = "Field `PCS24_PROT_CLR` writer - 24:24\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
pub type Pcs24ProtClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCS25_PROT_CLR` reader - 25:25\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
pub type Pcs25ProtClrR = crate::BitReader;
#[doc = "Field `PCS25_PROT_CLR` writer - 25:25\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
pub type Pcs25ProtClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCS26_PROT_CLR` reader - 26:26\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
pub type Pcs26ProtClrR = crate::BitReader;
#[doc = "Field `PCS26_PROT_CLR` writer - 26:26\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
pub type Pcs26ProtClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCS27_PROT_CLR` reader - 27:27\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
pub type Pcs27ProtClrR = crate::BitReader;
#[doc = "Field `PCS27_PROT_CLR` writer - 27:27\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
pub type Pcs27ProtClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCS28_PROT_CLR` reader - 28:28\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
pub type Pcs28ProtClrR = crate::BitReader;
#[doc = "Field `PCS28_PROT_CLR` writer - 28:28\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
pub type Pcs28ProtClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCS29_PROT_CLR` reader - 29:29\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
pub type Pcs29ProtClrR = crate::BitReader;
#[doc = "Field `PCS29_PROT_CLR` writer - 29:29\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
pub type Pcs29ProtClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCS30_PROT_CLR` reader - 30:30\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
pub type Pcs30ProtClrR = crate::BitReader;
#[doc = "Field `PCS30_PROT_CLR` writer - 30:30\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
pub type Pcs30ProtClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCS31_PROT_CLR` reader - 31:31\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
pub type Pcs31ProtClrR = crate::BitReader;
#[doc = "Field `PCS31_PROT_CLR` writer - 31:31\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
pub type Pcs31ProtClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
    #[inline(always)]
    pub fn pcs0_prot_clr(&self) -> Pcs0ProtClrR {
        Pcs0ProtClrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
    #[inline(always)]
    pub fn pcs1_prot_clr(&self) -> Pcs1ProtClrR {
        Pcs1ProtClrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
    #[inline(always)]
    pub fn pcs2_prot_clr(&self) -> Pcs2ProtClrR {
        Pcs2ProtClrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
    #[inline(always)]
    pub fn pcs3_prot_clr(&self) -> Pcs3ProtClrR {
        Pcs3ProtClrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
    #[inline(always)]
    pub fn pcs4_prot_clr(&self) -> Pcs4ProtClrR {
        Pcs4ProtClrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
    #[inline(always)]
    pub fn pcs5_prot_clr(&self) -> Pcs5ProtClrR {
        Pcs5ProtClrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
    #[inline(always)]
    pub fn pcs6_prot_clr(&self) -> Pcs6ProtClrR {
        Pcs6ProtClrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
    #[inline(always)]
    pub fn pcs7_prot_clr(&self) -> Pcs7ProtClrR {
        Pcs7ProtClrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
    #[inline(always)]
    pub fn pcs8_prot_clr(&self) -> Pcs8ProtClrR {
        Pcs8ProtClrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
    #[inline(always)]
    pub fn pcs9_prot_clr(&self) -> Pcs9ProtClrR {
        Pcs9ProtClrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
    #[inline(always)]
    pub fn pcs10_prot_clr(&self) -> Pcs10ProtClrR {
        Pcs10ProtClrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
    #[inline(always)]
    pub fn pcs11_prot_clr(&self) -> Pcs11ProtClrR {
        Pcs11ProtClrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
    #[inline(always)]
    pub fn pcs12_prot_clr(&self) -> Pcs12ProtClrR {
        Pcs12ProtClrR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
    #[inline(always)]
    pub fn pcs13_prot_clr(&self) -> Pcs13ProtClrR {
        Pcs13ProtClrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
    #[inline(always)]
    pub fn pcs14_prot_clr(&self) -> Pcs14ProtClrR {
        Pcs14ProtClrR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
    #[inline(always)]
    pub fn pcs15_prot_clr(&self) -> Pcs15ProtClrR {
        Pcs15ProtClrR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
    #[inline(always)]
    pub fn pcs16_prot_clr(&self) -> Pcs16ProtClrR {
        Pcs16ProtClrR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
    #[inline(always)]
    pub fn pcs17_prot_clr(&self) -> Pcs17ProtClrR {
        Pcs17ProtClrR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
    #[inline(always)]
    pub fn pcs18_prot_clr(&self) -> Pcs18ProtClrR {
        Pcs18ProtClrR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
    #[inline(always)]
    pub fn pcs19_prot_clr(&self) -> Pcs19ProtClrR {
        Pcs19ProtClrR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
    #[inline(always)]
    pub fn pcs20_prot_clr(&self) -> Pcs20ProtClrR {
        Pcs20ProtClrR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
    #[inline(always)]
    pub fn pcs21_prot_clr(&self) -> Pcs21ProtClrR {
        Pcs21ProtClrR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
    #[inline(always)]
    pub fn pcs22_prot_clr(&self) -> Pcs22ProtClrR {
        Pcs22ProtClrR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
    #[inline(always)]
    pub fn pcs23_prot_clr(&self) -> Pcs23ProtClrR {
        Pcs23ProtClrR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
    #[inline(always)]
    pub fn pcs24_prot_clr(&self) -> Pcs24ProtClrR {
        Pcs24ProtClrR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
    #[inline(always)]
    pub fn pcs25_prot_clr(&self) -> Pcs25ProtClrR {
        Pcs25ProtClrR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
    #[inline(always)]
    pub fn pcs26_prot_clr(&self) -> Pcs26ProtClrR {
        Pcs26ProtClrR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
    #[inline(always)]
    pub fn pcs27_prot_clr(&self) -> Pcs27ProtClrR {
        Pcs27ProtClrR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
    #[inline(always)]
    pub fn pcs28_prot_clr(&self) -> Pcs28ProtClrR {
        Pcs28ProtClrR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
    #[inline(always)]
    pub fn pcs29_prot_clr(&self) -> Pcs29ProtClrR {
        Pcs29ProtClrR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
    #[inline(always)]
    pub fn pcs30_prot_clr(&self) -> Pcs30ProtClrR {
        Pcs30ProtClrR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
    #[inline(always)]
    pub fn pcs31_prot_clr(&self) -> Pcs31ProtClrR {
        Pcs31ProtClrR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn pcs0_prot_clr(&mut self) -> Pcs0ProtClrW<Pmprotclr0Spec> {
        Pcs0ProtClrW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn pcs1_prot_clr(&mut self) -> Pcs1ProtClrW<Pmprotclr0Spec> {
        Pcs1ProtClrW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn pcs2_prot_clr(&mut self) -> Pcs2ProtClrW<Pmprotclr0Spec> {
        Pcs2ProtClrW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn pcs3_prot_clr(&mut self) -> Pcs3ProtClrW<Pmprotclr0Spec> {
        Pcs3ProtClrW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn pcs4_prot_clr(&mut self) -> Pcs4ProtClrW<Pmprotclr0Spec> {
        Pcs4ProtClrW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn pcs5_prot_clr(&mut self) -> Pcs5ProtClrW<Pmprotclr0Spec> {
        Pcs5ProtClrW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn pcs6_prot_clr(&mut self) -> Pcs6ProtClrW<Pmprotclr0Spec> {
        Pcs6ProtClrW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn pcs7_prot_clr(&mut self) -> Pcs7ProtClrW<Pmprotclr0Spec> {
        Pcs7ProtClrW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn pcs8_prot_clr(&mut self) -> Pcs8ProtClrW<Pmprotclr0Spec> {
        Pcs8ProtClrW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn pcs9_prot_clr(&mut self) -> Pcs9ProtClrW<Pmprotclr0Spec> {
        Pcs9ProtClrW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn pcs10_prot_clr(&mut self) -> Pcs10ProtClrW<Pmprotclr0Spec> {
        Pcs10ProtClrW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn pcs11_prot_clr(&mut self) -> Pcs11ProtClrW<Pmprotclr0Spec> {
        Pcs11ProtClrW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn pcs12_prot_clr(&mut self) -> Pcs12ProtClrW<Pmprotclr0Spec> {
        Pcs12ProtClrW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn pcs13_prot_clr(&mut self) -> Pcs13ProtClrW<Pmprotclr0Spec> {
        Pcs13ProtClrW::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn pcs14_prot_clr(&mut self) -> Pcs14ProtClrW<Pmprotclr0Spec> {
        Pcs14ProtClrW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn pcs15_prot_clr(&mut self) -> Pcs15ProtClrW<Pmprotclr0Spec> {
        Pcs15ProtClrW::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn pcs16_prot_clr(&mut self) -> Pcs16ProtClrW<Pmprotclr0Spec> {
        Pcs16ProtClrW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn pcs17_prot_clr(&mut self) -> Pcs17ProtClrW<Pmprotclr0Spec> {
        Pcs17ProtClrW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn pcs18_prot_clr(&mut self) -> Pcs18ProtClrW<Pmprotclr0Spec> {
        Pcs18ProtClrW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn pcs19_prot_clr(&mut self) -> Pcs19ProtClrW<Pmprotclr0Spec> {
        Pcs19ProtClrW::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn pcs20_prot_clr(&mut self) -> Pcs20ProtClrW<Pmprotclr0Spec> {
        Pcs20ProtClrW::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn pcs21_prot_clr(&mut self) -> Pcs21ProtClrW<Pmprotclr0Spec> {
        Pcs21ProtClrW::new(self, 21)
    }
    #[doc = "Bit 22 - 22:22\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn pcs22_prot_clr(&mut self) -> Pcs22ProtClrW<Pmprotclr0Spec> {
        Pcs22ProtClrW::new(self, 22)
    }
    #[doc = "Bit 23 - 23:23\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn pcs23_prot_clr(&mut self) -> Pcs23ProtClrW<Pmprotclr0Spec> {
        Pcs23ProtClrW::new(self, 23)
    }
    #[doc = "Bit 24 - 24:24\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn pcs24_prot_clr(&mut self) -> Pcs24ProtClrW<Pmprotclr0Spec> {
        Pcs24ProtClrW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn pcs25_prot_clr(&mut self) -> Pcs25ProtClrW<Pmprotclr0Spec> {
        Pcs25ProtClrW::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn pcs26_prot_clr(&mut self) -> Pcs26ProtClrW<Pmprotclr0Spec> {
        Pcs26ProtClrW::new(self, 26)
    }
    #[doc = "Bit 27 - 27:27\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn pcs27_prot_clr(&mut self) -> Pcs27ProtClrW<Pmprotclr0Spec> {
        Pcs27ProtClrW::new(self, 27)
    }
    #[doc = "Bit 28 - 28:28\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn pcs28_prot_clr(&mut self) -> Pcs28ProtClrW<Pmprotclr0Spec> {
        Pcs28ProtClrW::new(self, 28)
    }
    #[doc = "Bit 29 - 29:29\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn pcs29_prot_clr(&mut self) -> Pcs29ProtClrW<Pmprotclr0Spec> {
        Pcs29ProtClrW::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn pcs30_prot_clr(&mut self) -> Pcs30ProtClrW<Pmprotclr0Spec> {
        Pcs30ProtClrW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Readable in user and privileged modes 1 = The corresponding peripheral memory frame can be written to only in privileged mode but can be read in both user and privileged modes. 0 = The corresponding peripheral memory frame can be written to and read from in both user and privileged modes. Writable only in privileged mode 1 = Clears the corresponding bit in PMPROTCLR0 and PMPROTSET0 registers 0 = Has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn pcs31_prot_clr(&mut self) -> Pcs31ProtClrW<Pmprotclr0Spec> {
        Pcs31ProtClrW::new(self, 31)
    }
}
#[doc = "Clear-only register to protect PCS frames 0 to 31\n\nYou can [`read`](crate::Reg::read) this register and get [`pmprotclr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmprotclr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pmprotclr0Spec;
impl crate::RegisterSpec for Pmprotclr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmprotclr0::R`](R) reader structure"]
impl crate::Readable for Pmprotclr0Spec {}
#[doc = "`write(|w| ..)` method takes [`pmprotclr0::W`](W) writer structure"]
impl crate::Writable for Pmprotclr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMPROTCLR0 to value 0"]
impl crate::Resettable for Pmprotclr0Spec {
    const RESET_VALUE: u32 = 0;
}
