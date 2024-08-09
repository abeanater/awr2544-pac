#[doc = "Register `SPIPMCTRL` reader"]
pub type R = crate::R<SpipmctrlSpec>;
#[doc = "Register `SPIPMCTRL` writer"]
pub type W = crate::W<SpipmctrlSpec>;
#[doc = "Field `PMODE0` reader - 1:0\\]
Parallel mode bits determine whether the SPI/MibSPI operates with 1, 2 ,4 or 8 data lines for Data Format 0. 00 = normal operation / 1-data line (MMODE should be set to ΓÇ£000ΓÇ¥) 01 = 2-data line mode (MMODE should be set to ΓÇ£000ΓÇ¥) 10 = 4-data line mode (MMODE should be set to ΓÇ£000ΓÇ¥) 11 = 8-data line mode (MMODE should be set to ΓÇ£000ΓÇ¥)"]
pub type Pmode0R = crate::FieldReader;
#[doc = "Field `PMODE0` writer - 1:0\\]
Parallel mode bits determine whether the SPI/MibSPI operates with 1, 2 ,4 or 8 data lines for Data Format 0. 00 = normal operation / 1-data line (MMODE should be set to ΓÇ£000ΓÇ¥) 01 = 2-data line mode (MMODE should be set to ΓÇ£000ΓÇ¥) 10 = 4-data line mode (MMODE should be set to ΓÇ£000ΓÇ¥) 11 = 8-data line mode (MMODE should be set to ΓÇ£000ΓÇ¥)"]
pub type Pmode0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MMODE0` reader - 4:2\\]
These bits determine whether the SPI/MibSPI operates with 1, 2, 4, 5, or 6 data lines (if Modulo Option is supported by the module) for Data Format 0. 000 = 1-data line Mode - Default (PMODE should be set to ΓÇ£00ΓÇ¥) 001 = 2-data line Mode (PMODE should be set to ΓÇ£00ΓÇ¥) 010 = 3-data line mode (PMODE should be set to ΓÇ£00ΓÇ¥) 011 = 4-data line mode (PMODE should be set to ΓÇ£00ΓÇ¥) 100 = 5-data line mode (PMODE should be set to ΓÇ£00ΓÇ¥) 101 = 6-data line mode (PMODE should be set to ΓÇ£01ΓÇ¥) 110 = Reserved 111 = Reserved"]
pub type Mmode0R = crate::FieldReader;
#[doc = "Field `MMODE0` writer - 4:2\\]
These bits determine whether the SPI/MibSPI operates with 1, 2, 4, 5, or 6 data lines (if Modulo Option is supported by the module) for Data Format 0. 000 = 1-data line Mode - Default (PMODE should be set to ΓÇ£00ΓÇ¥) 001 = 2-data line Mode (PMODE should be set to ΓÇ£00ΓÇ¥) 010 = 3-data line mode (PMODE should be set to ΓÇ£00ΓÇ¥) 011 = 4-data line mode (PMODE should be set to ΓÇ£00ΓÇ¥) 100 = 5-data line mode (PMODE should be set to ΓÇ£00ΓÇ¥) 101 = 6-data line mode (PMODE should be set to ΓÇ£01ΓÇ¥) 110 = Reserved 111 = Reserved"]
pub type Mmode0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MODCLKPOL0` reader - 5:5\\]
Modulo mode SPICLK Polarity for Data Format 0. Determines the Polarity of the SPICLK in Modulo mode only. If MMODE\\[2:0\\]
bits are ΓÇ£000ΓÇ¥, this bit will be ignored. 0 = Normal SPICLK in all the modes. 1 = Polarity of the SPICLK will be inverted if Modulo mode is selected."]
pub type Modclkpol0R = crate::BitReader;
#[doc = "Field `MODCLKPOL0` writer - 5:5\\]
Modulo mode SPICLK Polarity for Data Format 0. Determines the Polarity of the SPICLK in Modulo mode only. If MMODE\\[2:0\\]
bits are ΓÇ£000ΓÇ¥, this bit will be ignored. 0 = Normal SPICLK in all the modes. 1 = Polarity of the SPICLK will be inverted if Modulo mode is selected."]
pub type Modclkpol0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSM_MODE0` reader - 6:6\\]
High Speed Modulo Mode control bit for Data Format 0. Controls whether the PMODE0 bits will result in Modulo Format data transfer or not. Refer to Section 3.26 for details about the HSM Mode. 0 = Normal mode - Normal Parallel mode if PMODE0 bits are non-zero. 1 = High Speed Modulo Mode. Data transfer will happen in Modulo Format if PMODE0 bits are non-zero."]
pub type HsmMode0R = crate::BitReader;
#[doc = "Field `HSM_MODE0` writer - 6:6\\]
High Speed Modulo Mode control bit for Data Format 0. Controls whether the PMODE0 bits will result in Modulo Format data transfer or not. Refer to Section 3.26 for details about the HSM Mode. 0 = Normal mode - Normal Parallel mode if PMODE0 bits are non-zero. 1 = High Speed Modulo Mode. Data transfer will happen in Modulo Format if PMODE0 bits are non-zero."]
pub type HsmMode0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU1` reader - 7:7\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu1R = crate::BitReader;
#[doc = "Field `NU1` writer - 7:7\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMODE1` reader - 9:8\\]
Parallel mode bits determine whether the SPI/MibSPI operates with 1, 2 ,4 or 8 data lines for Data Format 1. 00 = normal operation / 1-data line (MMODE should be set to ΓÇ£000ΓÇ¥) 01 = 2-data line mode (MMODE should be set to ΓÇ£000ΓÇ¥) 10 = 4-data line mode (MMODE should be set to ΓÇ£000ΓÇ¥) 11 = 8-data line mode (MMODE should be set to ΓÇ£000ΓÇ¥)"]
pub type Pmode1R = crate::FieldReader;
#[doc = "Field `PMODE1` writer - 9:8\\]
Parallel mode bits determine whether the SPI/MibSPI operates with 1, 2 ,4 or 8 data lines for Data Format 1. 00 = normal operation / 1-data line (MMODE should be set to ΓÇ£000ΓÇ¥) 01 = 2-data line mode (MMODE should be set to ΓÇ£000ΓÇ¥) 10 = 4-data line mode (MMODE should be set to ΓÇ£000ΓÇ¥) 11 = 8-data line mode (MMODE should be set to ΓÇ£000ΓÇ¥)"]
pub type Pmode1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MMODE1` reader - 12:10\\]
These bits determine whether the SPI/MibSPI operates with 1, 2, 4, 5, or 6 data lines (if Modulo Option is supported by the module) for Data Format 1. 000 = 1-data line Mode - Default (PMODE should be set to ΓÇ£00ΓÇ¥) 001 = 2-data line Mode (PMODE should be set to ΓÇ£00ΓÇ¥) 010 = 3-data line mode (PMODE should be set to ΓÇ£00ΓÇ¥) 011 = 4-data line mode (PMODE should be set to ΓÇ£00ΓÇ¥) 100 = 5-data line mode (PMODE should be set to ΓÇ£00ΓÇ¥) 101 = 6-data line mode (PMODE should be set to ΓÇ£01ΓÇ¥) 110 = Reserved 111 = Reserved"]
pub type Mmode1R = crate::FieldReader;
#[doc = "Field `MMODE1` writer - 12:10\\]
These bits determine whether the SPI/MibSPI operates with 1, 2, 4, 5, or 6 data lines (if Modulo Option is supported by the module) for Data Format 1. 000 = 1-data line Mode - Default (PMODE should be set to ΓÇ£00ΓÇ¥) 001 = 2-data line Mode (PMODE should be set to ΓÇ£00ΓÇ¥) 010 = 3-data line mode (PMODE should be set to ΓÇ£00ΓÇ¥) 011 = 4-data line mode (PMODE should be set to ΓÇ£00ΓÇ¥) 100 = 5-data line mode (PMODE should be set to ΓÇ£00ΓÇ¥) 101 = 6-data line mode (PMODE should be set to ΓÇ£01ΓÇ¥) 110 = Reserved 111 = Reserved"]
pub type Mmode1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MODCLKPOL1` reader - 13:13\\]
Modulo mode SPICLK Polarity for Data Format 1. Determines the Polarity of the SPICLK in Modulo mode only. If MMODE\\[2:0\\]
bits are ΓÇ£000ΓÇ¥, this bit will be ignored. 0 = Normal SPICLK in all the modes. 1 = Polarity of the SPICLK will be inverted if Modulo mode is selected."]
pub type Modclkpol1R = crate::BitReader;
#[doc = "Field `MODCLKPOL1` writer - 13:13\\]
Modulo mode SPICLK Polarity for Data Format 1. Determines the Polarity of the SPICLK in Modulo mode only. If MMODE\\[2:0\\]
bits are ΓÇ£000ΓÇ¥, this bit will be ignored. 0 = Normal SPICLK in all the modes. 1 = Polarity of the SPICLK will be inverted if Modulo mode is selected."]
pub type Modclkpol1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSM_MODE1` reader - 14:14\\]
High Speed Modulo Mode control bit for Data Format 1. Controls whether the PMODE1 bits will result in Modulo Format data transfer or not. Refer to Section 3.26 for details about the HSM Mode. 0 = Normal mode - Normal Parallel mode if PMODE1 bits are non-zero. 1 = High Speed Modulo Mode. Data transfer will happen in Modulo Format if PMODE1 bits are non-zero."]
pub type HsmMode1R = crate::BitReader;
#[doc = "Field `HSM_MODE1` writer - 14:14\\]
High Speed Modulo Mode control bit for Data Format 1. Controls whether the PMODE1 bits will result in Modulo Format data transfer or not. Refer to Section 3.26 for details about the HSM Mode. 0 = Normal mode - Normal Parallel mode if PMODE1 bits are non-zero. 1 = High Speed Modulo Mode. Data transfer will happen in Modulo Format if PMODE1 bits are non-zero."]
pub type HsmMode1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU2` reader - 15:15\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu2R = crate::BitReader;
#[doc = "Field `NU2` writer - 15:15\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMODE2` reader - 17:16\\]
Parallel mode bits determine whether the SPI/MibSPI operates with 1, 2 ,4 or 8 data lines for Data Format 2. 00 = normal operation / 1-data line (MMODE should be set to ΓÇ£000ΓÇ¥) 01 = 2-data line mode (MMODE should be set to ΓÇ£000ΓÇ¥)10 = 4-data line mode (MMODE should be set to ΓÇ£000ΓÇ¥) 11 = 8-data line mode (MMODE should be set to ΓÇ£000ΓÇ¥)"]
pub type Pmode2R = crate::FieldReader;
#[doc = "Field `PMODE2` writer - 17:16\\]
Parallel mode bits determine whether the SPI/MibSPI operates with 1, 2 ,4 or 8 data lines for Data Format 2. 00 = normal operation / 1-data line (MMODE should be set to ΓÇ£000ΓÇ¥) 01 = 2-data line mode (MMODE should be set to ΓÇ£000ΓÇ¥)10 = 4-data line mode (MMODE should be set to ΓÇ£000ΓÇ¥) 11 = 8-data line mode (MMODE should be set to ΓÇ£000ΓÇ¥)"]
pub type Pmode2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MMODE2` reader - 20:18\\]
These bits determine whether the SPI/MibSPI operates with 1, 2, 4, 5, or 6 data lines (if Modulo Option is supported by the module) for Data Format 2. 000 = 1-data line Mode - Default (PMODE should be set to ΓÇ£00ΓÇ¥) 001 = 2-data line Mode (PMODE should be set to ΓÇ£00ΓÇ¥) 010 = 3-data line mode (PMODE should be set to ΓÇ£00ΓÇ¥) 011 = 4-data line mode (PMODE should be set to ΓÇ£00ΓÇ¥) 100 = 5-data line mode (PMODE should be set to ΓÇ£00ΓÇ¥) 101 = 6-data line mode (PMODE should be set to ΓÇ£01ΓÇ¥) 110 = Reserved 111 = Reserved"]
pub type Mmode2R = crate::FieldReader;
#[doc = "Field `MMODE2` writer - 20:18\\]
These bits determine whether the SPI/MibSPI operates with 1, 2, 4, 5, or 6 data lines (if Modulo Option is supported by the module) for Data Format 2. 000 = 1-data line Mode - Default (PMODE should be set to ΓÇ£00ΓÇ¥) 001 = 2-data line Mode (PMODE should be set to ΓÇ£00ΓÇ¥) 010 = 3-data line mode (PMODE should be set to ΓÇ£00ΓÇ¥) 011 = 4-data line mode (PMODE should be set to ΓÇ£00ΓÇ¥) 100 = 5-data line mode (PMODE should be set to ΓÇ£00ΓÇ¥) 101 = 6-data line mode (PMODE should be set to ΓÇ£01ΓÇ¥) 110 = Reserved 111 = Reserved"]
pub type Mmode2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MODCLKPOL2` reader - 21:21\\]
Modulo mode SPICLK Polarity for Data Format 2. Determines the Polarity of the SPICLK in Modulo mode only. If MMODE\\[2:0\\]
bits are ΓÇ£000ΓÇ¥, this bit will be ignored. 0 = Normal SPICLK in all the modes. 1 = Polarity of the SPICLK will be inverted if Modulo mode is selected"]
pub type Modclkpol2R = crate::BitReader;
#[doc = "Field `MODCLKPOL2` writer - 21:21\\]
Modulo mode SPICLK Polarity for Data Format 2. Determines the Polarity of the SPICLK in Modulo mode only. If MMODE\\[2:0\\]
bits are ΓÇ£000ΓÇ¥, this bit will be ignored. 0 = Normal SPICLK in all the modes. 1 = Polarity of the SPICLK will be inverted if Modulo mode is selected"]
pub type Modclkpol2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSM_MODE2` reader - 22:22\\]
High Speed Modulo Mode control bit for Data Format 2. Controls whether the PMODE2 bits will result in Modulo Format data transfer or not. Refer to Section 3.26 for details about the HSM Mode. 0 = Normal mode - Normal Parallel mode if PMODE2 bits are non-zero. 1 = High Speed Modulo Mode. Data transfer will happen in Modulo Format if PMODE2 bits are non-zero"]
pub type HsmMode2R = crate::BitReader;
#[doc = "Field `HSM_MODE2` writer - 22:22\\]
High Speed Modulo Mode control bit for Data Format 2. Controls whether the PMODE2 bits will result in Modulo Format data transfer or not. Refer to Section 3.26 for details about the HSM Mode. 0 = Normal mode - Normal Parallel mode if PMODE2 bits are non-zero. 1 = High Speed Modulo Mode. Data transfer will happen in Modulo Format if PMODE2 bits are non-zero"]
pub type HsmMode2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU3` reader - 23:23\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu3R = crate::BitReader;
#[doc = "Field `NU3` writer - 23:23\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMODE3` reader - 25:24\\]
Parallel mode bits determine whether the SPI/MibSPI operates with 1, 2 ,4 or 8 data lines for Data Format 3. 00 = normal operation / 1-data line (MMODE should be set to ΓÇ£000ΓÇ¥) 01 = 2-data line mode (MMODE should be set to ΓÇ£000ΓÇ¥) 10 = 4-data line mode (MMODE should be set to ΓÇ£000ΓÇ¥) 11 = 8-data line mode (MMODE should be set to ΓÇ£000ΓÇ¥)"]
pub type Pmode3R = crate::FieldReader;
#[doc = "Field `PMODE3` writer - 25:24\\]
Parallel mode bits determine whether the SPI/MibSPI operates with 1, 2 ,4 or 8 data lines for Data Format 3. 00 = normal operation / 1-data line (MMODE should be set to ΓÇ£000ΓÇ¥) 01 = 2-data line mode (MMODE should be set to ΓÇ£000ΓÇ¥) 10 = 4-data line mode (MMODE should be set to ΓÇ£000ΓÇ¥) 11 = 8-data line mode (MMODE should be set to ΓÇ£000ΓÇ¥)"]
pub type Pmode3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MMODE3` reader - 28:26\\]
These bits determine whether the SPI/MibSPI operates with 1, 2, 4, 5, or 6 data lines (if Modulo Option is supported by the module) for Data Format 3.000 = Normal single dataline mode - Default (PMODE should be set to ΓÇ£00ΓÇ¥) 001 = 2-data line Mode (PMODE should be set to ΓÇ£00ΓÇ¥) 010 = 3-data line mode (PMODE should be set to ΓÇ£00ΓÇ¥) 011 = 4-data line mode (PMODE should be set to ΓÇ£00ΓÇ¥) 100 = 5-data line mode (PMODE should be set to ΓÇ£00ΓÇ¥) 101 = 6-data line mode (PMODE should be set to ΓÇ£01ΓÇ¥) 110 = Reserved 111 = Reserved"]
pub type Mmode3R = crate::FieldReader;
#[doc = "Field `MMODE3` writer - 28:26\\]
These bits determine whether the SPI/MibSPI operates with 1, 2, 4, 5, or 6 data lines (if Modulo Option is supported by the module) for Data Format 3.000 = Normal single dataline mode - Default (PMODE should be set to ΓÇ£00ΓÇ¥) 001 = 2-data line Mode (PMODE should be set to ΓÇ£00ΓÇ¥) 010 = 3-data line mode (PMODE should be set to ΓÇ£00ΓÇ¥) 011 = 4-data line mode (PMODE should be set to ΓÇ£00ΓÇ¥) 100 = 5-data line mode (PMODE should be set to ΓÇ£00ΓÇ¥) 101 = 6-data line mode (PMODE should be set to ΓÇ£01ΓÇ¥) 110 = Reserved 111 = Reserved"]
pub type Mmode3W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MODCLKPOL3` reader - 29:29\\]
Modulo mode SPICLK Polarity for Data Format 3 Determines the Polarity of the SPICLK in Modulo mode only. If MODULO MODE\\[2:0\\]
bits are ΓÇ£000ΓÇ¥, this bit will be ignored. 0 = Normal SPICLK in all the modes. 1 = Polarity of the SPICLK will be inverted if Modulo mode is selected"]
pub type Modclkpol3R = crate::BitReader;
#[doc = "Field `MODCLKPOL3` writer - 29:29\\]
Modulo mode SPICLK Polarity for Data Format 3 Determines the Polarity of the SPICLK in Modulo mode only. If MODULO MODE\\[2:0\\]
bits are ΓÇ£000ΓÇ¥, this bit will be ignored. 0 = Normal SPICLK in all the modes. 1 = Polarity of the SPICLK will be inverted if Modulo mode is selected"]
pub type Modclkpol3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSM_MODE3` reader - 30:30\\]
High Speed Modulo Mode control bit for Data Format 3. Controls whether the PMODE3 bits will result in Modulo Format data transfer or not. Refer to Section 3.26 for details about the HSM Mode. 0 = Normal mode - Normal Parallel mode if PMODE3 bits are non-zero. 1 = High Speed Modulo Mode. Data transfer will happen in Modulo Format if PMODE3 bits are non-zero"]
pub type HsmMode3R = crate::BitReader;
#[doc = "Field `HSM_MODE3` writer - 30:30\\]
High Speed Modulo Mode control bit for Data Format 3. Controls whether the PMODE3 bits will result in Modulo Format data transfer or not. Refer to Section 3.26 for details about the HSM Mode. 0 = Normal mode - Normal Parallel mode if PMODE3 bits are non-zero. 1 = High Speed Modulo Mode. Data transfer will happen in Modulo Format if PMODE3 bits are non-zero"]
pub type HsmMode3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU4` reader - 31:31\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu4R = crate::BitReader;
#[doc = "Field `NU4` writer - 31:31\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu4W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Parallel mode bits determine whether the SPI/MibSPI operates with 1, 2 ,4 or 8 data lines for Data Format 0. 00 = normal operation / 1-data line (MMODE should be set to ΓÇ£000ΓÇ¥) 01 = 2-data line mode (MMODE should be set to ΓÇ£000ΓÇ¥) 10 = 4-data line mode (MMODE should be set to ΓÇ£000ΓÇ¥) 11 = 8-data line mode (MMODE should be set to ΓÇ£000ΓÇ¥)"]
    #[inline(always)]
    pub fn pmode0(&self) -> Pmode0R {
        Pmode0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4 - 4:2\\]
These bits determine whether the SPI/MibSPI operates with 1, 2, 4, 5, or 6 data lines (if Modulo Option is supported by the module) for Data Format 0. 000 = 1-data line Mode - Default (PMODE should be set to ΓÇ£00ΓÇ¥) 001 = 2-data line Mode (PMODE should be set to ΓÇ£00ΓÇ¥) 010 = 3-data line mode (PMODE should be set to ΓÇ£00ΓÇ¥) 011 = 4-data line mode (PMODE should be set to ΓÇ£00ΓÇ¥) 100 = 5-data line mode (PMODE should be set to ΓÇ£00ΓÇ¥) 101 = 6-data line mode (PMODE should be set to ΓÇ£01ΓÇ¥) 110 = Reserved 111 = Reserved"]
    #[inline(always)]
    pub fn mmode0(&self) -> Mmode0R {
        Mmode0R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5 - 5:5\\]
Modulo mode SPICLK Polarity for Data Format 0. Determines the Polarity of the SPICLK in Modulo mode only. If MMODE\\[2:0\\]
bits are ΓÇ£000ΓÇ¥, this bit will be ignored. 0 = Normal SPICLK in all the modes. 1 = Polarity of the SPICLK will be inverted if Modulo mode is selected."]
    #[inline(always)]
    pub fn modclkpol0(&self) -> Modclkpol0R {
        Modclkpol0R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
High Speed Modulo Mode control bit for Data Format 0. Controls whether the PMODE0 bits will result in Modulo Format data transfer or not. Refer to Section 3.26 for details about the HSM Mode. 0 = Normal mode - Normal Parallel mode if PMODE0 bits are non-zero. 1 = High Speed Modulo Mode. Data transfer will happen in Modulo Format if PMODE0 bits are non-zero."]
    #[inline(always)]
    pub fn hsm_mode0(&self) -> HsmMode0R {
        HsmMode0R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Parallel mode bits determine whether the SPI/MibSPI operates with 1, 2 ,4 or 8 data lines for Data Format 1. 00 = normal operation / 1-data line (MMODE should be set to ΓÇ£000ΓÇ¥) 01 = 2-data line mode (MMODE should be set to ΓÇ£000ΓÇ¥) 10 = 4-data line mode (MMODE should be set to ΓÇ£000ΓÇ¥) 11 = 8-data line mode (MMODE should be set to ΓÇ£000ΓÇ¥)"]
    #[inline(always)]
    pub fn pmode1(&self) -> Pmode1R {
        Pmode1R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:12 - 12:10\\]
These bits determine whether the SPI/MibSPI operates with 1, 2, 4, 5, or 6 data lines (if Modulo Option is supported by the module) for Data Format 1. 000 = 1-data line Mode - Default (PMODE should be set to ΓÇ£00ΓÇ¥) 001 = 2-data line Mode (PMODE should be set to ΓÇ£00ΓÇ¥) 010 = 3-data line mode (PMODE should be set to ΓÇ£00ΓÇ¥) 011 = 4-data line mode (PMODE should be set to ΓÇ£00ΓÇ¥) 100 = 5-data line mode (PMODE should be set to ΓÇ£00ΓÇ¥) 101 = 6-data line mode (PMODE should be set to ΓÇ£01ΓÇ¥) 110 = Reserved 111 = Reserved"]
    #[inline(always)]
    pub fn mmode1(&self) -> Mmode1R {
        Mmode1R::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bit 13 - 13:13\\]
Modulo mode SPICLK Polarity for Data Format 1. Determines the Polarity of the SPICLK in Modulo mode only. If MMODE\\[2:0\\]
bits are ΓÇ£000ΓÇ¥, this bit will be ignored. 0 = Normal SPICLK in all the modes. 1 = Polarity of the SPICLK will be inverted if Modulo mode is selected."]
    #[inline(always)]
    pub fn modclkpol1(&self) -> Modclkpol1R {
        Modclkpol1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
High Speed Modulo Mode control bit for Data Format 1. Controls whether the PMODE1 bits will result in Modulo Format data transfer or not. Refer to Section 3.26 for details about the HSM Mode. 0 = Normal mode - Normal Parallel mode if PMODE1 bits are non-zero. 1 = High Speed Modulo Mode. Data transfer will happen in Modulo Format if PMODE1 bits are non-zero."]
    #[inline(always)]
    pub fn hsm_mode1(&self) -> HsmMode1R {
        HsmMode1R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    pub fn nu2(&self) -> Nu2R {
        Nu2R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Parallel mode bits determine whether the SPI/MibSPI operates with 1, 2 ,4 or 8 data lines for Data Format 2. 00 = normal operation / 1-data line (MMODE should be set to ΓÇ£000ΓÇ¥) 01 = 2-data line mode (MMODE should be set to ΓÇ£000ΓÇ¥)10 = 4-data line mode (MMODE should be set to ΓÇ£000ΓÇ¥) 11 = 8-data line mode (MMODE should be set to ΓÇ£000ΓÇ¥)"]
    #[inline(always)]
    pub fn pmode2(&self) -> Pmode2R {
        Pmode2R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:20 - 20:18\\]
These bits determine whether the SPI/MibSPI operates with 1, 2, 4, 5, or 6 data lines (if Modulo Option is supported by the module) for Data Format 2. 000 = 1-data line Mode - Default (PMODE should be set to ΓÇ£00ΓÇ¥) 001 = 2-data line Mode (PMODE should be set to ΓÇ£00ΓÇ¥) 010 = 3-data line mode (PMODE should be set to ΓÇ£00ΓÇ¥) 011 = 4-data line mode (PMODE should be set to ΓÇ£00ΓÇ¥) 100 = 5-data line mode (PMODE should be set to ΓÇ£00ΓÇ¥) 101 = 6-data line mode (PMODE should be set to ΓÇ£01ΓÇ¥) 110 = Reserved 111 = Reserved"]
    #[inline(always)]
    pub fn mmode2(&self) -> Mmode2R {
        Mmode2R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 21 - 21:21\\]
Modulo mode SPICLK Polarity for Data Format 2. Determines the Polarity of the SPICLK in Modulo mode only. If MMODE\\[2:0\\]
bits are ΓÇ£000ΓÇ¥, this bit will be ignored. 0 = Normal SPICLK in all the modes. 1 = Polarity of the SPICLK will be inverted if Modulo mode is selected"]
    #[inline(always)]
    pub fn modclkpol2(&self) -> Modclkpol2R {
        Modclkpol2R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
High Speed Modulo Mode control bit for Data Format 2. Controls whether the PMODE2 bits will result in Modulo Format data transfer or not. Refer to Section 3.26 for details about the HSM Mode. 0 = Normal mode - Normal Parallel mode if PMODE2 bits are non-zero. 1 = High Speed Modulo Mode. Data transfer will happen in Modulo Format if PMODE2 bits are non-zero"]
    #[inline(always)]
    pub fn hsm_mode2(&self) -> HsmMode2R {
        HsmMode2R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    pub fn nu3(&self) -> Nu3R {
        Nu3R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Parallel mode bits determine whether the SPI/MibSPI operates with 1, 2 ,4 or 8 data lines for Data Format 3. 00 = normal operation / 1-data line (MMODE should be set to ΓÇ£000ΓÇ¥) 01 = 2-data line mode (MMODE should be set to ΓÇ£000ΓÇ¥) 10 = 4-data line mode (MMODE should be set to ΓÇ£000ΓÇ¥) 11 = 8-data line mode (MMODE should be set to ΓÇ£000ΓÇ¥)"]
    #[inline(always)]
    pub fn pmode3(&self) -> Pmode3R {
        Pmode3R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:28 - 28:26\\]
These bits determine whether the SPI/MibSPI operates with 1, 2, 4, 5, or 6 data lines (if Modulo Option is supported by the module) for Data Format 3.000 = Normal single dataline mode - Default (PMODE should be set to ΓÇ£00ΓÇ¥) 001 = 2-data line Mode (PMODE should be set to ΓÇ£00ΓÇ¥) 010 = 3-data line mode (PMODE should be set to ΓÇ£00ΓÇ¥) 011 = 4-data line mode (PMODE should be set to ΓÇ£00ΓÇ¥) 100 = 5-data line mode (PMODE should be set to ΓÇ£00ΓÇ¥) 101 = 6-data line mode (PMODE should be set to ΓÇ£01ΓÇ¥) 110 = Reserved 111 = Reserved"]
    #[inline(always)]
    pub fn mmode3(&self) -> Mmode3R {
        Mmode3R::new(((self.bits >> 26) & 7) as u8)
    }
    #[doc = "Bit 29 - 29:29\\]
Modulo mode SPICLK Polarity for Data Format 3 Determines the Polarity of the SPICLK in Modulo mode only. If MODULO MODE\\[2:0\\]
bits are ΓÇ£000ΓÇ¥, this bit will be ignored. 0 = Normal SPICLK in all the modes. 1 = Polarity of the SPICLK will be inverted if Modulo mode is selected"]
    #[inline(always)]
    pub fn modclkpol3(&self) -> Modclkpol3R {
        Modclkpol3R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
High Speed Modulo Mode control bit for Data Format 3. Controls whether the PMODE3 bits will result in Modulo Format data transfer or not. Refer to Section 3.26 for details about the HSM Mode. 0 = Normal mode - Normal Parallel mode if PMODE3 bits are non-zero. 1 = High Speed Modulo Mode. Data transfer will happen in Modulo Format if PMODE3 bits are non-zero"]
    #[inline(always)]
    pub fn hsm_mode3(&self) -> HsmMode3R {
        HsmMode3R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    pub fn nu4(&self) -> Nu4R {
        Nu4R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Parallel mode bits determine whether the SPI/MibSPI operates with 1, 2 ,4 or 8 data lines for Data Format 0. 00 = normal operation / 1-data line (MMODE should be set to ΓÇ£000ΓÇ¥) 01 = 2-data line mode (MMODE should be set to ΓÇ£000ΓÇ¥) 10 = 4-data line mode (MMODE should be set to ΓÇ£000ΓÇ¥) 11 = 8-data line mode (MMODE should be set to ΓÇ£000ΓÇ¥)"]
    #[inline(always)]
    #[must_use]
    pub fn pmode0(&mut self) -> Pmode0W<SpipmctrlSpec> {
        Pmode0W::new(self, 0)
    }
    #[doc = "Bits 2:4 - 4:2\\]
These bits determine whether the SPI/MibSPI operates with 1, 2, 4, 5, or 6 data lines (if Modulo Option is supported by the module) for Data Format 0. 000 = 1-data line Mode - Default (PMODE should be set to ΓÇ£00ΓÇ¥) 001 = 2-data line Mode (PMODE should be set to ΓÇ£00ΓÇ¥) 010 = 3-data line mode (PMODE should be set to ΓÇ£00ΓÇ¥) 011 = 4-data line mode (PMODE should be set to ΓÇ£00ΓÇ¥) 100 = 5-data line mode (PMODE should be set to ΓÇ£00ΓÇ¥) 101 = 6-data line mode (PMODE should be set to ΓÇ£01ΓÇ¥) 110 = Reserved 111 = Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn mmode0(&mut self) -> Mmode0W<SpipmctrlSpec> {
        Mmode0W::new(self, 2)
    }
    #[doc = "Bit 5 - 5:5\\]
Modulo mode SPICLK Polarity for Data Format 0. Determines the Polarity of the SPICLK in Modulo mode only. If MMODE\\[2:0\\]
bits are ΓÇ£000ΓÇ¥, this bit will be ignored. 0 = Normal SPICLK in all the modes. 1 = Polarity of the SPICLK will be inverted if Modulo mode is selected."]
    #[inline(always)]
    #[must_use]
    pub fn modclkpol0(&mut self) -> Modclkpol0W<SpipmctrlSpec> {
        Modclkpol0W::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
High Speed Modulo Mode control bit for Data Format 0. Controls whether the PMODE0 bits will result in Modulo Format data transfer or not. Refer to Section 3.26 for details about the HSM Mode. 0 = Normal mode - Normal Parallel mode if PMODE0 bits are non-zero. 1 = High Speed Modulo Mode. Data transfer will happen in Modulo Format if PMODE0 bits are non-zero."]
    #[inline(always)]
    #[must_use]
    pub fn hsm_mode0(&mut self) -> HsmMode0W<SpipmctrlSpec> {
        HsmMode0W::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<SpipmctrlSpec> {
        Nu1W::new(self, 7)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Parallel mode bits determine whether the SPI/MibSPI operates with 1, 2 ,4 or 8 data lines for Data Format 1. 00 = normal operation / 1-data line (MMODE should be set to ΓÇ£000ΓÇ¥) 01 = 2-data line mode (MMODE should be set to ΓÇ£000ΓÇ¥) 10 = 4-data line mode (MMODE should be set to ΓÇ£000ΓÇ¥) 11 = 8-data line mode (MMODE should be set to ΓÇ£000ΓÇ¥)"]
    #[inline(always)]
    #[must_use]
    pub fn pmode1(&mut self) -> Pmode1W<SpipmctrlSpec> {
        Pmode1W::new(self, 8)
    }
    #[doc = "Bits 10:12 - 12:10\\]
These bits determine whether the SPI/MibSPI operates with 1, 2, 4, 5, or 6 data lines (if Modulo Option is supported by the module) for Data Format 1. 000 = 1-data line Mode - Default (PMODE should be set to ΓÇ£00ΓÇ¥) 001 = 2-data line Mode (PMODE should be set to ΓÇ£00ΓÇ¥) 010 = 3-data line mode (PMODE should be set to ΓÇ£00ΓÇ¥) 011 = 4-data line mode (PMODE should be set to ΓÇ£00ΓÇ¥) 100 = 5-data line mode (PMODE should be set to ΓÇ£00ΓÇ¥) 101 = 6-data line mode (PMODE should be set to ΓÇ£01ΓÇ¥) 110 = Reserved 111 = Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn mmode1(&mut self) -> Mmode1W<SpipmctrlSpec> {
        Mmode1W::new(self, 10)
    }
    #[doc = "Bit 13 - 13:13\\]
Modulo mode SPICLK Polarity for Data Format 1. Determines the Polarity of the SPICLK in Modulo mode only. If MMODE\\[2:0\\]
bits are ΓÇ£000ΓÇ¥, this bit will be ignored. 0 = Normal SPICLK in all the modes. 1 = Polarity of the SPICLK will be inverted if Modulo mode is selected."]
    #[inline(always)]
    #[must_use]
    pub fn modclkpol1(&mut self) -> Modclkpol1W<SpipmctrlSpec> {
        Modclkpol1W::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
High Speed Modulo Mode control bit for Data Format 1. Controls whether the PMODE1 bits will result in Modulo Format data transfer or not. Refer to Section 3.26 for details about the HSM Mode. 0 = Normal mode - Normal Parallel mode if PMODE1 bits are non-zero. 1 = High Speed Modulo Mode. Data transfer will happen in Modulo Format if PMODE1 bits are non-zero."]
    #[inline(always)]
    #[must_use]
    pub fn hsm_mode1(&mut self) -> HsmMode1W<SpipmctrlSpec> {
        HsmMode1W::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn nu2(&mut self) -> Nu2W<SpipmctrlSpec> {
        Nu2W::new(self, 15)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Parallel mode bits determine whether the SPI/MibSPI operates with 1, 2 ,4 or 8 data lines for Data Format 2. 00 = normal operation / 1-data line (MMODE should be set to ΓÇ£000ΓÇ¥) 01 = 2-data line mode (MMODE should be set to ΓÇ£000ΓÇ¥)10 = 4-data line mode (MMODE should be set to ΓÇ£000ΓÇ¥) 11 = 8-data line mode (MMODE should be set to ΓÇ£000ΓÇ¥)"]
    #[inline(always)]
    #[must_use]
    pub fn pmode2(&mut self) -> Pmode2W<SpipmctrlSpec> {
        Pmode2W::new(self, 16)
    }
    #[doc = "Bits 18:20 - 20:18\\]
These bits determine whether the SPI/MibSPI operates with 1, 2, 4, 5, or 6 data lines (if Modulo Option is supported by the module) for Data Format 2. 000 = 1-data line Mode - Default (PMODE should be set to ΓÇ£00ΓÇ¥) 001 = 2-data line Mode (PMODE should be set to ΓÇ£00ΓÇ¥) 010 = 3-data line mode (PMODE should be set to ΓÇ£00ΓÇ¥) 011 = 4-data line mode (PMODE should be set to ΓÇ£00ΓÇ¥) 100 = 5-data line mode (PMODE should be set to ΓÇ£00ΓÇ¥) 101 = 6-data line mode (PMODE should be set to ΓÇ£01ΓÇ¥) 110 = Reserved 111 = Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn mmode2(&mut self) -> Mmode2W<SpipmctrlSpec> {
        Mmode2W::new(self, 18)
    }
    #[doc = "Bit 21 - 21:21\\]
Modulo mode SPICLK Polarity for Data Format 2. Determines the Polarity of the SPICLK in Modulo mode only. If MMODE\\[2:0\\]
bits are ΓÇ£000ΓÇ¥, this bit will be ignored. 0 = Normal SPICLK in all the modes. 1 = Polarity of the SPICLK will be inverted if Modulo mode is selected"]
    #[inline(always)]
    #[must_use]
    pub fn modclkpol2(&mut self) -> Modclkpol2W<SpipmctrlSpec> {
        Modclkpol2W::new(self, 21)
    }
    #[doc = "Bit 22 - 22:22\\]
High Speed Modulo Mode control bit for Data Format 2. Controls whether the PMODE2 bits will result in Modulo Format data transfer or not. Refer to Section 3.26 for details about the HSM Mode. 0 = Normal mode - Normal Parallel mode if PMODE2 bits are non-zero. 1 = High Speed Modulo Mode. Data transfer will happen in Modulo Format if PMODE2 bits are non-zero"]
    #[inline(always)]
    #[must_use]
    pub fn hsm_mode2(&mut self) -> HsmMode2W<SpipmctrlSpec> {
        HsmMode2W::new(self, 22)
    }
    #[doc = "Bit 23 - 23:23\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn nu3(&mut self) -> Nu3W<SpipmctrlSpec> {
        Nu3W::new(self, 23)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Parallel mode bits determine whether the SPI/MibSPI operates with 1, 2 ,4 or 8 data lines for Data Format 3. 00 = normal operation / 1-data line (MMODE should be set to ΓÇ£000ΓÇ¥) 01 = 2-data line mode (MMODE should be set to ΓÇ£000ΓÇ¥) 10 = 4-data line mode (MMODE should be set to ΓÇ£000ΓÇ¥) 11 = 8-data line mode (MMODE should be set to ΓÇ£000ΓÇ¥)"]
    #[inline(always)]
    #[must_use]
    pub fn pmode3(&mut self) -> Pmode3W<SpipmctrlSpec> {
        Pmode3W::new(self, 24)
    }
    #[doc = "Bits 26:28 - 28:26\\]
These bits determine whether the SPI/MibSPI operates with 1, 2, 4, 5, or 6 data lines (if Modulo Option is supported by the module) for Data Format 3.000 = Normal single dataline mode - Default (PMODE should be set to ΓÇ£00ΓÇ¥) 001 = 2-data line Mode (PMODE should be set to ΓÇ£00ΓÇ¥) 010 = 3-data line mode (PMODE should be set to ΓÇ£00ΓÇ¥) 011 = 4-data line mode (PMODE should be set to ΓÇ£00ΓÇ¥) 100 = 5-data line mode (PMODE should be set to ΓÇ£00ΓÇ¥) 101 = 6-data line mode (PMODE should be set to ΓÇ£01ΓÇ¥) 110 = Reserved 111 = Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn mmode3(&mut self) -> Mmode3W<SpipmctrlSpec> {
        Mmode3W::new(self, 26)
    }
    #[doc = "Bit 29 - 29:29\\]
Modulo mode SPICLK Polarity for Data Format 3 Determines the Polarity of the SPICLK in Modulo mode only. If MODULO MODE\\[2:0\\]
bits are ΓÇ£000ΓÇ¥, this bit will be ignored. 0 = Normal SPICLK in all the modes. 1 = Polarity of the SPICLK will be inverted if Modulo mode is selected"]
    #[inline(always)]
    #[must_use]
    pub fn modclkpol3(&mut self) -> Modclkpol3W<SpipmctrlSpec> {
        Modclkpol3W::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
High Speed Modulo Mode control bit for Data Format 3. Controls whether the PMODE3 bits will result in Modulo Format data transfer or not. Refer to Section 3.26 for details about the HSM Mode. 0 = Normal mode - Normal Parallel mode if PMODE3 bits are non-zero. 1 = High Speed Modulo Mode. Data transfer will happen in Modulo Format if PMODE3 bits are non-zero"]
    #[inline(always)]
    #[must_use]
    pub fn hsm_mode3(&mut self) -> HsmMode3W<SpipmctrlSpec> {
        HsmMode3W::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn nu4(&mut self) -> Nu4W<SpipmctrlSpec> {
        Nu4W::new(self, 31)
    }
}
#[doc = "SPI/MibSPI Parallel/Modulo Mode Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spipmctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipmctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpipmctrlSpec;
impl crate::RegisterSpec for SpipmctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipmctrl::R`](R) reader structure"]
impl crate::Readable for SpipmctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`spipmctrl::W`](W) writer structure"]
impl crate::Writable for SpipmctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPIPMCTRL to value 0"]
impl crate::Resettable for SpipmctrlSpec {
    const RESET_VALUE: u32 = 0;
}
