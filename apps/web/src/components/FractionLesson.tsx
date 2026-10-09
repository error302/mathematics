import React from 'react';
import { LessonViewer } from './LessonViewer';

interface FractionLessonProps {
  onStartPractice: (templateId?: string) => void;
  selectedLessonId?: string;
}

export const FractionLesson: React.FC<FractionLessonProps> = ({
  onStartPractice,
  selectedLessonId = "foundation.fractions.compare",
}) => {
  return (
    <LessonViewer
      initialLessonId={selectedLessonId}
      onStartPractice={(tmpl) => onStartPractice(tmpl)}
    />
  );
};
