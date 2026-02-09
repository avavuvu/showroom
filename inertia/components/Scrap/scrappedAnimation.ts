
export const animateScrap = async (editorParent: HTMLElement, targetElementRef: HTMLElement) => {
    const { gsap } = await import('gsap');
    const { SplitText } = await import('gsap/SplitText');

    gsap.registerPlugin(SplitText);

    const split = new SplitText(editorParent, { type: 'words' });

    const targetRect = targetElementRef.getBoundingClientRect();
    const targetX = targetRect.left + targetRect.width / 2;
    const targetY = targetRect.top + targetRect.height / 2;

    const tl = gsap.timeline();

    tl
        .to(".bin-icon", {
            y: "0",
            duration: 1
        })
        .to(split.words, {
            x: (_, target) => {
                const rect = target.getBoundingClientRect();
                return targetX - (rect.left + rect.width / 2);
            },
            y: (_, target) => {
                const rect = target.getBoundingClientRect();
                return targetY - (rect.top + rect.height / 2);
            },
            duration: 0.8,
            stagger: 0.02,
            ease: 'power2.in'
        })
        .to(split.words, {
            opacity: 0,
            duration: 0.2,
        }, '-=0.1')
        .to(".evil-shadow", {
            opacity: 0,
            duration: 2,
        })
        .to(".encouraging-words", {
            opacity: 1,
            duration: 4,
        }, '+=1')

}