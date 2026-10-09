import React, { useEffect, useRef } from 'react';
import katex from 'katex';

interface MathViewProps {
  math: string;
  display?: boolean;
}

export const MathView: React.FC<MathViewProps> = ({ math, display = false }) => {
  const containerRef = useRef<HTMLSpanElement>(null);

  useEffect(() => {
    if (containerRef.current) {
      try {
        katex.render(math, containerRef.current, {
          displayMode: display,
          throwOnError: false,
          trust: false, // Security constraint from Section 24.3
          output: 'htmlAndMathml',
        });
      } catch (err) {
        containerRef.current.innerText = math;
      }
    }
  }, [math, display]);

  return <span ref={containerRef} className="axiom-math-view" />;
};
