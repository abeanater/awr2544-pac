#[doc = "Register `BSS_CONTROL` reader"]
pub type R = crate::R<BssControlSpec>;
#[doc = "Register `BSS_CONTROL` writer"]
pub type W = crate::W<BssControlSpec>;
#[doc = "Field `bootmode` reader - 11:0\\]
writing 12'h000 : selects the normal boot mode for CR4. 12'h111 : selects the FW dev mode for CR4 12'h222 : selects the orbit mode for CR4 12'h333 : selects the 14xx ROM swap mode"]
pub type BootmodeR = crate::FieldReader<u16>;
#[doc = "Field `bootmode` writer - 11:0\\]
writing 12'h000 : selects the normal boot mode for CR4. 12'h111 : selects the FW dev mode for CR4 12'h222 : selects the orbit mode for CR4 12'h333 : selects the 14xx ROM swap mode"]
pub type BootmodeW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `wfi_override` reader - 18:16\\]
writing 3'b111 overrides the WFI signal from CR4 and asserts it."]
pub type WfiOverrideR = crate::FieldReader;
#[doc = "Field `wfi_override` writer - 18:16\\]
writing 3'b111 overrides the WFI signal from CR4 and asserts it."]
pub type WfiOverrideW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `halt` reader - 26:24\\]
writing 3'b000 unhalts BSS. This is expected to be writen only once per processor reset cycle."]
pub type HaltR = crate::FieldReader;
#[doc = "Field `halt` writer - 26:24\\]
writing 3'b000 unhalts BSS. This is expected to be writen only once per processor reset cycle."]
pub type HaltW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `dss_l3_access` reader - 30:28\\]
writing 3'b111 allocates DSS_L3_BANKB1 256KB as TCM for RSS_CR4"]
pub type DssL3AccessR = crate::FieldReader;
#[doc = "Field `dss_l3_access` writer - 30:28\\]
writing 3'b111 allocates DSS_L3_BANKB1 256KB as TCM for RSS_CR4"]
pub type DssL3AccessW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
writing 12'h000 : selects the normal boot mode for CR4. 12'h111 : selects the FW dev mode for CR4 12'h222 : selects the orbit mode for CR4 12'h333 : selects the 14xx ROM swap mode"]
    #[inline(always)]
    pub fn bootmode(&self) -> BootmodeR {
        BootmodeR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:18 - 18:16\\]
writing 3'b111 overrides the WFI signal from CR4 and asserts it."]
    #[inline(always)]
    pub fn wfi_override(&self) -> WfiOverrideR {
        WfiOverrideR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:26 - 26:24\\]
writing 3'b000 unhalts BSS. This is expected to be writen only once per processor reset cycle."]
    #[inline(always)]
    pub fn halt(&self) -> HaltR {
        HaltR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - 30:28\\]
writing 3'b111 allocates DSS_L3_BANKB1 256KB as TCM for RSS_CR4"]
    #[inline(always)]
    pub fn dss_l3_access(&self) -> DssL3AccessR {
        DssL3AccessR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
writing 12'h000 : selects the normal boot mode for CR4. 12'h111 : selects the FW dev mode for CR4 12'h222 : selects the orbit mode for CR4 12'h333 : selects the 14xx ROM swap mode"]
    #[inline(always)]
    #[must_use]
    pub fn bootmode(&mut self) -> BootmodeW<BssControlSpec> {
        BootmodeW::new(self, 0)
    }
    #[doc = "Bits 16:18 - 18:16\\]
writing 3'b111 overrides the WFI signal from CR4 and asserts it."]
    #[inline(always)]
    #[must_use]
    pub fn wfi_override(&mut self) -> WfiOverrideW<BssControlSpec> {
        WfiOverrideW::new(self, 16)
    }
    #[doc = "Bits 24:26 - 26:24\\]
writing 3'b000 unhalts BSS. This is expected to be writen only once per processor reset cycle."]
    #[inline(always)]
    #[must_use]
    pub fn halt(&mut self) -> HaltW<BssControlSpec> {
        HaltW::new(self, 24)
    }
    #[doc = "Bits 28:30 - 30:28\\]
writing 3'b111 allocates DSS_L3_BANKB1 256KB as TCM for RSS_CR4"]
    #[inline(always)]
    #[must_use]
    pub fn dss_l3_access(&mut self) -> DssL3AccessW<BssControlSpec> {
        DssL3AccessW::new(self, 28)
    }
}
#[doc = "BSS_CONTROL\n\nYou can [`read`](crate::Reg::read) this register and get [`bss_control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bss_control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BssControlSpec;
impl crate::RegisterSpec for BssControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bss_control::R`](R) reader structure"]
impl crate::Readable for BssControlSpec {}
#[doc = "`write(|w| ..)` method takes [`bss_control::W`](W) writer structure"]
impl crate::Writable for BssControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BSS_CONTROL to value 0"]
impl crate::Resettable for BssControlSpec {
    const RESET_VALUE: u32 = 0;
}
