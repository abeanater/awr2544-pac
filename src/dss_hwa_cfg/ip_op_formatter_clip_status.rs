#[doc = "Register `IP_OP_FORMATTER_CLIP_STATUS` reader"]
pub type R = crate::R<IpOpFormatterClipStatusSpec>;
#[doc = "Register `IP_OP_FORMATTER_CLIP_STATUS` writer"]
pub type W = crate::W<IpOpFormatterClipStatusSpec>;
#[doc = "Field `ip_formatter_clip_status` reader - 0:0\\]
Indicates the input formatter clip status"]
pub type IpFormatterClipStatusR = crate::BitReader;
#[doc = "Field `ip_formatter_clip_status` writer - 0:0\\]
Indicates the input formatter clip status"]
pub type IpFormatterClipStatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `op_formatter_clip_status` reader - 16:16\\]
Indicates the output formatter clip status"]
pub type OpFormatterClipStatusR = crate::BitReader;
#[doc = "Field `op_formatter_clip_status` writer - 16:16\\]
Indicates the output formatter clip status"]
pub type OpFormatterClipStatusW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Indicates the input formatter clip status"]
    #[inline(always)]
    pub fn ip_formatter_clip_status(&self) -> IpFormatterClipStatusR {
        IpFormatterClipStatusR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Indicates the output formatter clip status"]
    #[inline(always)]
    pub fn op_formatter_clip_status(&self) -> OpFormatterClipStatusR {
        OpFormatterClipStatusR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Indicates the input formatter clip status"]
    #[inline(always)]
    #[must_use]
    pub fn ip_formatter_clip_status(
        &mut self,
    ) -> IpFormatterClipStatusW<IpOpFormatterClipStatusSpec> {
        IpFormatterClipStatusW::new(self, 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Indicates the output formatter clip status"]
    #[inline(always)]
    #[must_use]
    pub fn op_formatter_clip_status(
        &mut self,
    ) -> OpFormatterClipStatusW<IpOpFormatterClipStatusSpec> {
        OpFormatterClipStatusW::new(self, 16)
    }
}
#[doc = "IP_OP_FORMATTER_CLIP_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`ip_op_formatter_clip_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ip_op_formatter_clip_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IpOpFormatterClipStatusSpec;
impl crate::RegisterSpec for IpOpFormatterClipStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ip_op_formatter_clip_status::R`](R) reader structure"]
impl crate::Readable for IpOpFormatterClipStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`ip_op_formatter_clip_status::W`](W) writer structure"]
impl crate::Writable for IpOpFormatterClipStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IP_OP_FORMATTER_CLIP_STATUS to value 0"]
impl crate::Resettable for IpOpFormatterClipStatusSpec {
    const RESET_VALUE: u32 = 0;
}
