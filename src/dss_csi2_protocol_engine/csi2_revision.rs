#[doc = "Register `CSI2_REVISION` reader"]
pub type R = crate::R<Csi2RevisionSpec>;
#[doc = "Register `CSI2_REVISION` writer"]
pub type W = crate::W<Csi2RevisionSpec>;
#[doc = "Field `REV` reader - 7:0\\]
IP revision \\[7:4\\]
Major revision \\[3:0\\]
Minor revision"]
pub type RevR = crate::FieldReader;
#[doc = "Field `REV` writer - 7:0\\]
IP revision \\[7:4\\]
Major revision \\[3:0\\]
Minor revision"]
pub type RevW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
IP revision \\[7:4\\]
Major revision \\[3:0\\]
Minor revision"]
    #[inline(always)]
    pub fn rev(&self) -> RevR {
        RevR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
IP revision \\[7:4\\]
Major revision \\[3:0\\]
Minor revision"]
    #[inline(always)]
    #[must_use]
    pub fn rev(&mut self) -> RevW<Csi2RevisionSpec> {
        RevW::new(self, 0)
    }
}
#[doc = "MODULE REVISION This register contains the IP revision code in binary coded digital. For example, we have: 0x01 = revision 0.1 and 0x21 = revision 2.1\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_revision::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_revision::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csi2RevisionSpec;
impl crate::RegisterSpec for Csi2RevisionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csi2_revision::R`](R) reader structure"]
impl crate::Readable for Csi2RevisionSpec {}
#[doc = "`write(|w| ..)` method takes [`csi2_revision::W`](W) writer structure"]
impl crate::Writable for Csi2RevisionSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSI2_REVISION to value 0"]
impl crate::Resettable for Csi2RevisionSpec {
    const RESET_VALUE: u32 = 0;
}
