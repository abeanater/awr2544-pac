#[doc = "Register `PAR_ECC_CTRL` reader"]
pub type R = crate::R<ParEccCtrlSpec>;
#[doc = "Register `PAR_ECC_CTRL` writer"]
pub type W = crate::W<ParEccCtrlSpec>;
#[doc = "Field `EDEN` reader - 3:0\\]
Error Detection Enable These bits enable Parity/ECC Error Detection. Write: 0101: Disables Parity/ECC Error Detection Logic(default) Others : Enables Parity/ECC Error Detection Logic. Read: Returns the current value of this field"]
pub type EdenR = crate::FieldReader;
#[doc = "Field `EDEN` writer - 3:0\\]
Error Detection Enable These bits enable Parity/ECC Error Detection. Write: 0101: Disables Parity/ECC Error Detection Logic(default) Others : Enables Parity/ECC Error Detection Logic. Read: Returns the current value of this field"]
pub type EdenW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NU1` reader - 7:4\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu1R = crate::FieldReader;
#[doc = "Field `NU1` writer - 7:4\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PTESTEN` reader - 8:8\\]
Parity/ECC memory Test Enable. This bit, maps the parity/ecc bits corresponding to Multibuffer RAM locations into the peripheral RAM frame to make them accessible by the CPU. User and privilege mode (read): 0 = parity/ecc bits are not memory mapped 1 = parity/ecc bits are memory mapped Privilege mode (write): 0 = disable memory mapping of Parity/ECC locations 1 = enable memory mapping of Parity/ECC locations"]
pub type PtestenR = crate::BitReader;
#[doc = "Field `PTESTEN` writer - 8:8\\]
Parity/ECC memory Test Enable. This bit, maps the parity/ecc bits corresponding to Multibuffer RAM locations into the peripheral RAM frame to make them accessible by the CPU. User and privilege mode (read): 0 = parity/ecc bits are not memory mapped 1 = parity/ecc bits are memory mapped Privilege mode (write): 0 = disable memory mapping of Parity/ECC locations 1 = enable memory mapping of Parity/ECC locations"]
pub type PtestenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU2` reader - 15:9\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu2R = crate::FieldReader;
#[doc = "Field `NU2` writer - 15:9\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `EDAC_MODE` reader - 19:16\\]
Error Detection And Correction Mode These bits determine whether Single Bit Errors (SBE) detected by the SECDED block will be corrected or not. Write: 0101 - Disable correction of SBE detected by the SECDED block 1010 - Enable correction of SBE detected by the SECDED block All other values - writes are ignored and the values are not updated into this field. The state of the feature remains unchanged. Read: Returns the current value of the field"]
pub type EdacModeR = crate::FieldReader;
#[doc = "Field `EDAC_MODE` writer - 19:16\\]
Error Detection And Correction Mode These bits determine whether Single Bit Errors (SBE) detected by the SECDED block will be corrected or not. Write: 0101 - Disable correction of SBE detected by the SECDED block 1010 - Enable correction of SBE detected by the SECDED block All other values - writes are ignored and the values are not updated into this field. The state of the feature remains unchanged. Read: Returns the current value of the field"]
pub type EdacModeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NU3` reader - 23:20\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu3R = crate::FieldReader;
#[doc = "Field `NU3` writer - 23:20\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SBE_EVT_EN` reader - 27:24\\]
Single Bit Error Event Enable This bit controls the generation of Error signaling (on MIBSPI_SBERR port) whenever a Single Bit Errors (SBE) is detected on TXRAM/RXRAM. This signal can be used to generate interrupt if required. Write: 0101 - Disable Error Event indication upon detection of SBE on TXRAM/RXRAM 1010 - Enable Error Event upon detection of SBE on TXRAM/RXRAM All other values - writes are ignored and the values are not updated into this field. The state of the feature remains unchanged. Read: Returns the current value of the field"]
pub type SbeEvtEnR = crate::FieldReader;
#[doc = "Field `SBE_EVT_EN` writer - 27:24\\]
Single Bit Error Event Enable This bit controls the generation of Error signaling (on MIBSPI_SBERR port) whenever a Single Bit Errors (SBE) is detected on TXRAM/RXRAM. This signal can be used to generate interrupt if required. Write: 0101 - Disable Error Event indication upon detection of SBE on TXRAM/RXRAM 1010 - Enable Error Event upon detection of SBE on TXRAM/RXRAM All other values - writes are ignored and the values are not updated into this field. The state of the feature remains unchanged. Read: Returns the current value of the field"]
pub type SbeEvtEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NU4` reader - 31:28\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu4R = crate::FieldReader;
#[doc = "Field `NU4` writer - 31:28\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Error Detection Enable These bits enable Parity/ECC Error Detection. Write: 0101: Disables Parity/ECC Error Detection Logic(default) Others : Enables Parity/ECC Error Detection Logic. Read: Returns the current value of this field"]
    #[inline(always)]
    pub fn eden(&self) -> EdenR {
        EdenR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Parity/ECC memory Test Enable. This bit, maps the parity/ecc bits corresponding to Multibuffer RAM locations into the peripheral RAM frame to make them accessible by the CPU. User and privilege mode (read): 0 = parity/ecc bits are not memory mapped 1 = parity/ecc bits are memory mapped Privilege mode (write): 0 = disable memory mapping of Parity/ECC locations 1 = enable memory mapping of Parity/ECC locations"]
    #[inline(always)]
    pub fn ptesten(&self) -> PtestenR {
        PtestenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    pub fn nu2(&self) -> Nu2R {
        Nu2R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Error Detection And Correction Mode These bits determine whether Single Bit Errors (SBE) detected by the SECDED block will be corrected or not. Write: 0101 - Disable correction of SBE detected by the SECDED block 1010 - Enable correction of SBE detected by the SECDED block All other values - writes are ignored and the values are not updated into this field. The state of the feature remains unchanged. Read: Returns the current value of the field"]
    #[inline(always)]
    pub fn edac_mode(&self) -> EdacModeR {
        EdacModeR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    pub fn nu3(&self) -> Nu3R {
        Nu3R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Single Bit Error Event Enable This bit controls the generation of Error signaling (on MIBSPI_SBERR port) whenever a Single Bit Errors (SBE) is detected on TXRAM/RXRAM. This signal can be used to generate interrupt if required. Write: 0101 - Disable Error Event indication upon detection of SBE on TXRAM/RXRAM 1010 - Enable Error Event upon detection of SBE on TXRAM/RXRAM All other values - writes are ignored and the values are not updated into this field. The state of the feature remains unchanged. Read: Returns the current value of the field"]
    #[inline(always)]
    pub fn sbe_evt_en(&self) -> SbeEvtEnR {
        SbeEvtEnR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    pub fn nu4(&self) -> Nu4R {
        Nu4R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Error Detection Enable These bits enable Parity/ECC Error Detection. Write: 0101: Disables Parity/ECC Error Detection Logic(default) Others : Enables Parity/ECC Error Detection Logic. Read: Returns the current value of this field"]
    #[inline(always)]
    #[must_use]
    pub fn eden(&mut self) -> EdenW<ParEccCtrlSpec> {
        EdenW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<ParEccCtrlSpec> {
        Nu1W::new(self, 4)
    }
    #[doc = "Bit 8 - 8:8\\]
Parity/ECC memory Test Enable. This bit, maps the parity/ecc bits corresponding to Multibuffer RAM locations into the peripheral RAM frame to make them accessible by the CPU. User and privilege mode (read): 0 = parity/ecc bits are not memory mapped 1 = parity/ecc bits are memory mapped Privilege mode (write): 0 = disable memory mapping of Parity/ECC locations 1 = enable memory mapping of Parity/ECC locations"]
    #[inline(always)]
    #[must_use]
    pub fn ptesten(&mut self) -> PtestenW<ParEccCtrlSpec> {
        PtestenW::new(self, 8)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn nu2(&mut self) -> Nu2W<ParEccCtrlSpec> {
        Nu2W::new(self, 9)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Error Detection And Correction Mode These bits determine whether Single Bit Errors (SBE) detected by the SECDED block will be corrected or not. Write: 0101 - Disable correction of SBE detected by the SECDED block 1010 - Enable correction of SBE detected by the SECDED block All other values - writes are ignored and the values are not updated into this field. The state of the feature remains unchanged. Read: Returns the current value of the field"]
    #[inline(always)]
    #[must_use]
    pub fn edac_mode(&mut self) -> EdacModeW<ParEccCtrlSpec> {
        EdacModeW::new(self, 16)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn nu3(&mut self) -> Nu3W<ParEccCtrlSpec> {
        Nu3W::new(self, 20)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Single Bit Error Event Enable This bit controls the generation of Error signaling (on MIBSPI_SBERR port) whenever a Single Bit Errors (SBE) is detected on TXRAM/RXRAM. This signal can be used to generate interrupt if required. Write: 0101 - Disable Error Event indication upon detection of SBE on TXRAM/RXRAM 1010 - Enable Error Event upon detection of SBE on TXRAM/RXRAM All other values - writes are ignored and the values are not updated into this field. The state of the feature remains unchanged. Read: Returns the current value of the field"]
    #[inline(always)]
    #[must_use]
    pub fn sbe_evt_en(&mut self) -> SbeEvtEnW<ParEccCtrlSpec> {
        SbeEvtEnW::new(self, 24)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn nu4(&mut self) -> Nu4W<ParEccCtrlSpec> {
        Nu4W::new(self, 28)
    }
}
#[doc = "Parity/ECC Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`par_ecc_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`par_ecc_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ParEccCtrlSpec;
impl crate::RegisterSpec for ParEccCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`par_ecc_ctrl::R`](R) reader structure"]
impl crate::Readable for ParEccCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`par_ecc_ctrl::W`](W) writer structure"]
impl crate::Writable for ParEccCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PAR_ECC_CTRL to value 0"]
impl crate::Resettable for ParEccCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
