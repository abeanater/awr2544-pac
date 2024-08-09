#[doc = "Register `MIBSPIE` reader"]
pub type R = crate::R<MibspieSpec>;
#[doc = "Register `MIBSPIE` writer"]
pub type W = crate::W<MibspieSpec>;
#[doc = "Field `MSPIENA` reader - 0:0\\]
Multibuffer mode Enable. After power-up or reset MSPIENA remains cleared, which means that the MibSPI runs in compatibility mode by default. If Multibuffer mode is desired, this register should be configured first after configuring the SPIGCR0 register. Unless MSPIENA is set to ΓÇÿ1ΓÇÖ, the Multibuffer mode registers are not writable. Refer to Section 3.3 for the grouping of registers into Compatibility mode and Multibuffer mode. 1 =The MibSPI is configured to run in MibSPI mode (Multibuffer mode). In this mode the additional features are available. 0 =The MibSPI runs in compatibility mode, i.e. in this mode the MibSPI is fully code compliant to the standard TMS470 Platform SPI. No Multibuffer feature is supported"]
pub type MspienaR = crate::BitReader;
#[doc = "Field `MSPIENA` writer - 0:0\\]
Multibuffer mode Enable. After power-up or reset MSPIENA remains cleared, which means that the MibSPI runs in compatibility mode by default. If Multibuffer mode is desired, this register should be configured first after configuring the SPIGCR0 register. Unless MSPIENA is set to ΓÇÿ1ΓÇÖ, the Multibuffer mode registers are not writable. Refer to Section 3.3 for the grouping of registers into Compatibility mode and Multibuffer mode. 1 =The MibSPI is configured to run in MibSPI mode (Multibuffer mode). In this mode the additional features are available. 0 =The MibSPI runs in compatibility mode, i.e. in this mode the MibSPI is fully code compliant to the standard TMS470 Platform SPI. No Multibuffer feature is supported"]
pub type MspienaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU1` reader - 7:1\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu1R = crate::FieldReader;
#[doc = "Field `NU1` writer - 7:1\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `EXTENDED_BUF_ENA` reader - 11:8\\]
Enables the support for 256 buffers. By default MibSPI supports up to 128 buffers for both TX and RX. It is also possible to extend the support to 256 buffers as a parameterized implementation. This field can be used to enable/disable the support for Extended Buffers. This Enable field is implemented only if ΓÇ£EXTENDED_BUFΓÇ¥ parameter is set to ΓÇÿ1ΓÇÖ. If the parameter is set to ΓÇÿ0ΓÇÖ, this field is read-only and reads the disable value. Write (Privilege mode only) 1010 - Enable the Extended Buffer mode - up to 256 buffers can be used 0101 - Disable the Extended Buffer mode - MibSPI supports only 128 buffers All other values - writes are ignored and the values are not updated into this field. The state of the feature remains unchanged. Read (both privilege and user modes) 1010 - Extended Buffer mode is enabled - up to 256 buffers can be used 0101 - Extended Buffer mode is disabled - MibSPI supports only 128 buffers"]
pub type ExtendedBufEnaR = crate::FieldReader;
#[doc = "Field `EXTENDED_BUF_ENA` writer - 11:8\\]
Enables the support for 256 buffers. By default MibSPI supports up to 128 buffers for both TX and RX. It is also possible to extend the support to 256 buffers as a parameterized implementation. This field can be used to enable/disable the support for Extended Buffers. This Enable field is implemented only if ΓÇ£EXTENDED_BUFΓÇ¥ parameter is set to ΓÇÿ1ΓÇÖ. If the parameter is set to ΓÇÿ0ΓÇÖ, this field is read-only and reads the disable value. Write (Privilege mode only) 1010 - Enable the Extended Buffer mode - up to 256 buffers can be used 0101 - Disable the Extended Buffer mode - MibSPI supports only 128 buffers All other values - writes are ignored and the values are not updated into this field. The state of the feature remains unchanged. Read (both privilege and user modes) 1010 - Extended Buffer mode is enabled - up to 256 buffers can be used 0101 - Extended Buffer mode is disabled - MibSPI supports only 128 buffers"]
pub type ExtendedBufEnaW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NU2` reader - 15:12\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect"]
pub type Nu2R = crate::FieldReader;
#[doc = "Field `NU2` writer - 15:12\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect"]
pub type Nu2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RXRAMACCESS` reader - 16:16\\]
Receive RAM Access control Bit. During normal operating mode of MibSPI, the Receive Data/Status portion of Multibuffer RAM is read-only. To enable testing of Data Integrity checks of Receive RAM, a special read/write access control is provided through this bit. 0 = The RX portion of Multibuffer RAM is not writable by the CPU. That is, portion of Multibuffer RAM, addressed by an offset of 0x200-0x3FF is write protected. 1 = The whole of Multibuffer RAM is fully accessible for read/write by the CPU."]
pub type RxramaccessR = crate::BitReader;
#[doc = "Field `RXRAMACCESS` writer - 16:16\\]
Receive RAM Access control Bit. During normal operating mode of MibSPI, the Receive Data/Status portion of Multibuffer RAM is read-only. To enable testing of Data Integrity checks of Receive RAM, a special read/write access control is provided through this bit. 0 = The RX portion of Multibuffer RAM is not writable by the CPU. That is, portion of Multibuffer RAM, addressed by an offset of 0x200-0x3FF is write protected. 1 = The whole of Multibuffer RAM is fully accessible for read/write by the CPU."]
pub type RxramaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU3` reader - 31:17\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu3R = crate::FieldReader<u16>;
#[doc = "Field `NU3` writer - 31:17\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu3W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Multibuffer mode Enable. After power-up or reset MSPIENA remains cleared, which means that the MibSPI runs in compatibility mode by default. If Multibuffer mode is desired, this register should be configured first after configuring the SPIGCR0 register. Unless MSPIENA is set to ΓÇÿ1ΓÇÖ, the Multibuffer mode registers are not writable. Refer to Section 3.3 for the grouping of registers into Compatibility mode and Multibuffer mode. 1 =The MibSPI is configured to run in MibSPI mode (Multibuffer mode). In this mode the additional features are available. 0 =The MibSPI runs in compatibility mode, i.e. in this mode the MibSPI is fully code compliant to the standard TMS470 Platform SPI. No Multibuffer feature is supported"]
    #[inline(always)]
    pub fn mspiena(&self) -> MspienaR {
        MspienaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Enables the support for 256 buffers. By default MibSPI supports up to 128 buffers for both TX and RX. It is also possible to extend the support to 256 buffers as a parameterized implementation. This field can be used to enable/disable the support for Extended Buffers. This Enable field is implemented only if ΓÇ£EXTENDED_BUFΓÇ¥ parameter is set to ΓÇÿ1ΓÇÖ. If the parameter is set to ΓÇÿ0ΓÇÖ, this field is read-only and reads the disable value. Write (Privilege mode only) 1010 - Enable the Extended Buffer mode - up to 256 buffers can be used 0101 - Disable the Extended Buffer mode - MibSPI supports only 128 buffers All other values - writes are ignored and the values are not updated into this field. The state of the feature remains unchanged. Read (both privilege and user modes) 1010 - Extended Buffer mode is enabled - up to 256 buffers can be used 0101 - Extended Buffer mode is disabled - MibSPI supports only 128 buffers"]
    #[inline(always)]
    pub fn extended_buf_ena(&self) -> ExtendedBufEnaR {
        ExtendedBufEnaR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect"]
    #[inline(always)]
    pub fn nu2(&self) -> Nu2R {
        Nu2R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Receive RAM Access control Bit. During normal operating mode of MibSPI, the Receive Data/Status portion of Multibuffer RAM is read-only. To enable testing of Data Integrity checks of Receive RAM, a special read/write access control is provided through this bit. 0 = The RX portion of Multibuffer RAM is not writable by the CPU. That is, portion of Multibuffer RAM, addressed by an offset of 0x200-0x3FF is write protected. 1 = The whole of Multibuffer RAM is fully accessible for read/write by the CPU."]
    #[inline(always)]
    pub fn rxramaccess(&self) -> RxramaccessR {
        RxramaccessR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:31 - 31:17\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    pub fn nu3(&self) -> Nu3R {
        Nu3R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Multibuffer mode Enable. After power-up or reset MSPIENA remains cleared, which means that the MibSPI runs in compatibility mode by default. If Multibuffer mode is desired, this register should be configured first after configuring the SPIGCR0 register. Unless MSPIENA is set to ΓÇÿ1ΓÇÖ, the Multibuffer mode registers are not writable. Refer to Section 3.3 for the grouping of registers into Compatibility mode and Multibuffer mode. 1 =The MibSPI is configured to run in MibSPI mode (Multibuffer mode). In this mode the additional features are available. 0 =The MibSPI runs in compatibility mode, i.e. in this mode the MibSPI is fully code compliant to the standard TMS470 Platform SPI. No Multibuffer feature is supported"]
    #[inline(always)]
    #[must_use]
    pub fn mspiena(&mut self) -> MspienaW<MibspieSpec> {
        MspienaW::new(self, 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<MibspieSpec> {
        Nu1W::new(self, 1)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Enables the support for 256 buffers. By default MibSPI supports up to 128 buffers for both TX and RX. It is also possible to extend the support to 256 buffers as a parameterized implementation. This field can be used to enable/disable the support for Extended Buffers. This Enable field is implemented only if ΓÇ£EXTENDED_BUFΓÇ¥ parameter is set to ΓÇÿ1ΓÇÖ. If the parameter is set to ΓÇÿ0ΓÇÖ, this field is read-only and reads the disable value. Write (Privilege mode only) 1010 - Enable the Extended Buffer mode - up to 256 buffers can be used 0101 - Disable the Extended Buffer mode - MibSPI supports only 128 buffers All other values - writes are ignored and the values are not updated into this field. The state of the feature remains unchanged. Read (both privilege and user modes) 1010 - Extended Buffer mode is enabled - up to 256 buffers can be used 0101 - Extended Buffer mode is disabled - MibSPI supports only 128 buffers"]
    #[inline(always)]
    #[must_use]
    pub fn extended_buf_ena(&mut self) -> ExtendedBufEnaW<MibspieSpec> {
        ExtendedBufEnaW::new(self, 8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect"]
    #[inline(always)]
    #[must_use]
    pub fn nu2(&mut self) -> Nu2W<MibspieSpec> {
        Nu2W::new(self, 12)
    }
    #[doc = "Bit 16 - 16:16\\]
Receive RAM Access control Bit. During normal operating mode of MibSPI, the Receive Data/Status portion of Multibuffer RAM is read-only. To enable testing of Data Integrity checks of Receive RAM, a special read/write access control is provided through this bit. 0 = The RX portion of Multibuffer RAM is not writable by the CPU. That is, portion of Multibuffer RAM, addressed by an offset of 0x200-0x3FF is write protected. 1 = The whole of Multibuffer RAM is fully accessible for read/write by the CPU."]
    #[inline(always)]
    #[must_use]
    pub fn rxramaccess(&mut self) -> RxramaccessW<MibspieSpec> {
        RxramaccessW::new(self, 16)
    }
    #[doc = "Bits 17:31 - 31:17\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn nu3(&mut self) -> Nu3W<MibspieSpec> {
        Nu3W::new(self, 17)
    }
}
#[doc = "MibSPI Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mibspie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mibspie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MibspieSpec;
impl crate::RegisterSpec for MibspieSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mibspie::R`](R) reader structure"]
impl crate::Readable for MibspieSpec {}
#[doc = "`write(|w| ..)` method takes [`mibspie::W`](W) writer structure"]
impl crate::Writable for MibspieSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MIBSPIE to value 0"]
impl crate::Resettable for MibspieSpec {
    const RESET_VALUE: u32 = 0;
}
