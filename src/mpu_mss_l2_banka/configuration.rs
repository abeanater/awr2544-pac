#[doc = "Register `Configuration` reader"]
pub type R = crate::R<ConfigurationSpec>;
#[doc = "Register `Configuration` writer"]
pub type W = crate::W<ConfigurationSpec>;
#[doc = "Field `assumed_allowed` reader - 0:0\\]
Assumed allowed mode. 0 = assumed disallowed, 1 = assumed allowed"]
pub type AssumedAllowedR = crate::BitReader;
#[doc = "Field `assumed_allowed` writer - 0:0\\]
Assumed allowed mode. 0 = assumed disallowed, 1 = assumed allowed"]
pub type AssumedAllowedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `num_fixed_aids` reader - 15:12\\]
Number of supported AIDs. 0 = no specific AIDs supported (all treated equally) N = PrivIDs from 0 to N-1 supported, others use AIDX"]
pub type NumFixedAidsR = crate::FieldReader;
#[doc = "Field `num_fixed_aids` writer - 15:12\\]
Number of supported AIDs. 0 = no specific AIDs supported (all treated equally) N = PrivIDs from 0 to N-1 supported, others use AIDX"]
pub type NumFixedAidsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `num_prog` reader - 19:16\\]
Number of programmable address ranges.Value is determined by configuration"]
pub type NumProgR = crate::FieldReader;
#[doc = "Field `num_prog` writer - 19:16\\]
Number of programmable address ranges.Value is determined by configuration"]
pub type NumProgW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `num_fixed` reader - 23:20\\]
Number of fixed address ranges Configurable as 0 or 1."]
pub type NumFixedR = crate::FieldReader;
#[doc = "Field `num_fixed` writer - 23:20\\]
Number of fixed address ranges Configurable as 0 or 1."]
pub type NumFixedW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `address_align` reader - 31:24\\]
Address alignment for range checking."]
pub type AddressAlignR = crate::FieldReader;
#[doc = "Field `address_align` writer - 31:24\\]
Address alignment for range checking."]
pub type AddressAlignW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Assumed allowed mode. 0 = assumed disallowed, 1 = assumed allowed"]
    #[inline(always)]
    pub fn assumed_allowed(&self) -> AssumedAllowedR {
        AssumedAllowedR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Number of supported AIDs. 0 = no specific AIDs supported (all treated equally) N = PrivIDs from 0 to N-1 supported, others use AIDX"]
    #[inline(always)]
    pub fn num_fixed_aids(&self) -> NumFixedAidsR {
        NumFixedAidsR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Number of programmable address ranges.Value is determined by configuration"]
    #[inline(always)]
    pub fn num_prog(&self) -> NumProgR {
        NumProgR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Number of fixed address ranges Configurable as 0 or 1."]
    #[inline(always)]
    pub fn num_fixed(&self) -> NumFixedR {
        NumFixedR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Address alignment for range checking."]
    #[inline(always)]
    pub fn address_align(&self) -> AddressAlignR {
        AddressAlignR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Assumed allowed mode. 0 = assumed disallowed, 1 = assumed allowed"]
    #[inline(always)]
    #[must_use]
    pub fn assumed_allowed(&mut self) -> AssumedAllowedW<ConfigurationSpec> {
        AssumedAllowedW::new(self, 0)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Number of supported AIDs. 0 = no specific AIDs supported (all treated equally) N = PrivIDs from 0 to N-1 supported, others use AIDX"]
    #[inline(always)]
    #[must_use]
    pub fn num_fixed_aids(&mut self) -> NumFixedAidsW<ConfigurationSpec> {
        NumFixedAidsW::new(self, 12)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Number of programmable address ranges.Value is determined by configuration"]
    #[inline(always)]
    #[must_use]
    pub fn num_prog(&mut self) -> NumProgW<ConfigurationSpec> {
        NumProgW::new(self, 16)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Number of fixed address ranges Configurable as 0 or 1."]
    #[inline(always)]
    #[must_use]
    pub fn num_fixed(&mut self) -> NumFixedW<ConfigurationSpec> {
        NumFixedW::new(self, 20)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Address alignment for range checking."]
    #[inline(always)]
    #[must_use]
    pub fn address_align(&mut self) -> AddressAlignW<ConfigurationSpec> {
        AddressAlignW::new(self, 24)
    }
}
#[doc = "Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`configuration::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`configuration::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigurationSpec;
impl crate::RegisterSpec for ConfigurationSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`configuration::R`](R) reader structure"]
impl crate::Readable for ConfigurationSpec {}
#[doc = "`write(|w| ..)` method takes [`configuration::W`](W) writer structure"]
impl crate::Writable for ConfigurationSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets Configuration to value 0"]
impl crate::Resettable for ConfigurationSpec {
    const RESET_VALUE: u32 = 0;
}
