import React from 'react';
import { CheckCircle, ChevronRight, Layers, Lock, UploadCloud } from 'lucide-react';
import { Curriculum } from '../types';

interface CurriculumViewProps {
  curricula: Curriculum[];
  onSelectTopic: (topicTitle: string) => void;
}

export const CurriculumView: React.FC<CurriculumViewProps> = ({
  curricula,
  onSelectTopic,
}) => {
  // Active curriculum from props or default
  const activeCurriculum = curricula[0] || {
    id: '1',
    title: 'Rust Systems Engineering & AI Tooling',
    description: 'Master memory safety, concurrency, low-level OS interfaces, and production AI pipelines.',
    status: 'ACTIVE',
    target_career_role: 'Systems & AI Engineer',
    estimated_total_hours: 120,
  };

  const modules = [
    {
      id: 'm1',
      title: 'Module 1: Ownership, Borrowing & Lifetimes',
      hours: '20 hrs',
      isUnlocked: true,
      topics: [
        { id: 't1', title: 'Memory Layout & Stack vs Heap', time: '60 min', status: 'MASTERED' },
        { id: 't2', title: 'The Borrow Checker & Aliasing Rules', time: '90 min', status: 'MASTERED' },
        { id: 't3', title: 'Generic Lifetimes & Subtyping', time: '120 min', status: 'IN_PROGRESS' },
      ],
    },
    {
      id: 'm2',
      title: 'Module 2: Multithreading & Asynchronous Tokio',
      hours: '35 hrs',
      isUnlocked: true,
      topics: [
        { id: 't4', title: 'Message Passing with Crossbeam & Channels', time: '90 min', status: 'UNLOCKED' },
        { id: 't5', title: 'Async/Await Internals & Wakers', time: '120 min', status: 'LOCKED' },
        { id: 't6', title: 'Actors & Concurrency Patterns', time: '90 min', status: 'LOCKED' },
      ],
    },
    {
      id: 'm3',
      title: 'Module 3: AI Inference & Embedding Systems',
      hours: '45 hrs',
      isUnlocked: false,
      topics: [
        { id: 't7', title: 'ONNX Runtime & Tensor Interop in Rust', time: '120 min', status: 'LOCKED' },
        { id: 't8', title: 'Vector Similarity & Annoy/HNSW Indices', time: '120 min', status: 'LOCKED' },
      ],
    },
  ];

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
        <div>
          <h2 className="text-2xl font-bold text-white tracking-tight">Curriculum Intelligence & Roadmap</h2>
          <p className="text-sm text-slate-400 mt-1">
            Track structured engineering objectives, prerequisites, and verified competence.
          </p>
        </div>

        <div className="flex items-center space-x-3">
          <button className="flex items-center space-x-1.5 bg-[#161a22] hover:bg-[#1e232e] text-slate-200 text-xs font-medium px-4 py-2.5 rounded-full border border-white/[0.08] transition-colors">
            <UploadCloud className="w-3.5 h-3.5 text-amber-400" />
            <span>Ingest Syllabus (PDF/MD)</span>
          </button>
        </div>
      </div>

      {/* Curriculum Banner */}
      <div className="rounded-3xl bg-[#12151b] border border-white/[0.07] p-6 shadow-md relative overflow-hidden">
        <div className="flex items-start justify-between">
          <div className="space-y-2">
            <div className="flex items-center space-x-2">
              <span className="text-xs font-semibold px-2.5 py-0.5 rounded-full bg-amber-500/10 text-amber-400 border border-amber-500/20">
                Active Roadmap
              </span>
              <span className="text-xs text-slate-400">• {activeCurriculum.target_career_role}</span>
            </div>
            <h3 className="text-xl font-bold text-white">{activeCurriculum.title}</h3>
            <p className="text-sm text-slate-300 max-w-2xl">{activeCurriculum.description}</p>
          </div>
          <div className="text-right hidden sm:block">
            <span className="text-xs text-slate-400">Total Estimated Effort</span>
            <div className="text-2xl font-bold text-amber-400">{activeCurriculum.estimated_total_hours} hrs</div>
          </div>
        </div>
      </div>

      {/* Modules List */}
      <div className="space-y-4">
        {modules.map((mod) => (
          <div
            key={mod.id}
            className="rounded-2xl bg-[#14171f] border border-white/[0.06] p-5 space-y-3 transition-colors hover:border-white/[0.12]"
          >
            <div className="flex items-center justify-between">
              <div className="flex items-center space-x-3">
                <div className="w-8 h-8 rounded-xl bg-white/[0.05] flex items-center justify-center text-slate-300">
                  <Layers className="w-4 h-4 text-amber-400" />
                </div>
                <div>
                  <h4 className="text-sm font-semibold text-white">{mod.title}</h4>
                  <span className="text-xs text-slate-400">{mod.hours}</span>
                </div>
              </div>

              {!mod.isUnlocked && (
                <span className="flex items-center space-x-1 text-xs text-slate-400 bg-white/[0.04] px-2.5 py-1 rounded-full">
                  <Lock className="w-3 h-3" />
                  <span>Prerequisites pending</span>
                </span>
              )}
            </div>

            {/* Topics */}
            <div className="grid grid-cols-1 md:grid-cols-3 gap-3 pt-2">
              {mod.topics.map((topic) => {
                const isMastered = topic.status === 'MASTERED';
                const isLocked = topic.status === 'LOCKED';

                return (
                  <div
                    key={topic.id}
                    onClick={() => !isLocked && onSelectTopic(topic.title)}
                    className={`p-3.5 rounded-xl border transition-all flex flex-col justify-between ${
                      isLocked
                        ? 'bg-black/20 border-white/[0.03] opacity-50 cursor-not-allowed'
                        : 'bg-[#1b202a]/80 hover:bg-[#222835] border-white/[0.08] cursor-pointer hover:shadow-md'
                    }`}
                  >
                    <div className="flex items-start justify-between">
                      <span className="text-xs font-medium text-slate-200 line-clamp-2">
                        {topic.title}
                      </span>
                      {isMastered ? (
                        <CheckCircle className="w-3.5 h-3.5 text-emerald-400 flex-shrink-0 ml-2" />
                      ) : isLocked ? (
                        <Lock className="w-3.5 h-3.5 text-slate-500 flex-shrink-0 ml-2" />
                      ) : (
                        <ChevronRight className="w-3.5 h-3.5 text-slate-400 flex-shrink-0 ml-2" />
                      )}
                    </div>

                    <div className="flex items-center justify-between mt-3 text-[11px] text-slate-400">
                      <span>{topic.time}</span>
                      <span
                        className={`font-medium ${
                          isMastered
                            ? 'text-emerald-400'
                            : topic.status === 'IN_PROGRESS'
                            ? 'text-amber-400'
                            : 'text-slate-400'
                        }`}
                      >
                        {topic.status}
                      </span>
                    </div>
                  </div>
                );
              })}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};
