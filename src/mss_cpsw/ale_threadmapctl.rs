#[doc = "Register `ALE_THREADMAPCTL` reader"]
pub type R = crate::R<AleThreadmapctlSpec>;
#[doc = "Register `ALE_THREADMAPCTL` writer"]
pub type W = crate::W<AleThreadmapctlSpec>;
#[doc = "Field `CLASSIFIER_INDEX_THIS` reader - 1:0\\]
Classifier Index - This is the classifier index entry that the thread enable and thread value will be read or written by the ~bTHREADMAPVAL register."]
pub type ClassifierIndexThisR = crate::FieldReader;
#[doc = "Field `CLASSIFIER_INDEX_THIS` writer - 1:0\\]
Classifier Index - This is the classifier index entry that the thread enable and thread value will be read or written by the ~bTHREADMAPVAL register."]
pub type ClassifierIndexThisW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Classifier Index - This is the classifier index entry that the thread enable and thread value will be read or written by the ~bTHREADMAPVAL register."]
    #[inline(always)]
    pub fn classifier_index_this(&self) -> ClassifierIndexThisR {
        ClassifierIndexThisR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Classifier Index - This is the classifier index entry that the thread enable and thread value will be read or written by the ~bTHREADMAPVAL register."]
    #[inline(always)]
    #[must_use]
    pub fn classifier_index_this(&mut self) -> ClassifierIndexThisW<AleThreadmapctlSpec> {
        ClassifierIndexThisW::new(self, 0)
    }
}
#[doc = "The THREAD Mapping Control register allows the highest matched classifier to return a particular thread ID for traffic sent to the host. This allows particular classifier matched traffic to be placed an a particular hosts queue.\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_threadmapctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_threadmapctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AleThreadmapctlSpec;
impl crate::RegisterSpec for AleThreadmapctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ale_threadmapctl::R`](R) reader structure"]
impl crate::Readable for AleThreadmapctlSpec {}
#[doc = "`write(|w| ..)` method takes [`ale_threadmapctl::W`](W) writer structure"]
impl crate::Writable for AleThreadmapctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALE_THREADMAPCTL to value 0"]
impl crate::Resettable for AleThreadmapctlSpec {
    const RESET_VALUE: u32 = 0;
}
