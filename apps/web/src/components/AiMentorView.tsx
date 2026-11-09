import React, { useState } from 'react';
import { Bot, Send, Sparkles } from 'lucide-react';

interface AiMentorViewProps {
  currentTopic: string;
}

export const AiMentorView: React.FC<AiMentorViewProps> = ({ currentTopic }) => {
  const [messages, setMessages] = useState<Array<{ role: 'user' | 'assistant'; content: string }>>([
    {
      role: 'assistant',
      content: `Hello! I am your WORKSPACE Engineering Mentor. We are currently focusing on **${currentTopic}**.\n\nWould you like a step-by-step tutorial, practical code examples, or to initiate a 5-question Checkpoint Assessment?`,
    },
  ]);
  const [input, setInput] = useState('');
  const [assessmentResult, setAssessmentResult] = useState<{
    status: 'MASTERED' | 'NEEDS_REVISION' | 'NOT_MASTERED';
    score: number;
    gaps: string[];
    feedback: string;
  } | null>(null);

  const handleSend = () => {
    if (!input.trim()) return;

    const userMsg = input.trim();
    setMessages((prev) => [...prev, { role: 'user', content: userMsg }]);
    setInput('');

    // Simulate context-aware AI response based on engineering guidelines
    setTimeout(() => {
      let reply = '';
      if (userMsg.toLowerCase().includes('checkpoint') || userMsg.toLowerCase().includes('assessment')) {
        reply = `Starting Checkpoint Assessment for **${currentTopic}**:\n\n1. Explain how the Borrow Checker prevents data races without a garbage collector.\n2. In what situation does an aliasing violation occur when using \`&mut T\`?\n3. Implement a thread-safe counter using \`Arc<AtomicUsize>\` or \`Arc<Mutex<u32>>\`.\n\nPlease type your code and explanation below!`;
      } else {
        reply = `In **${currentTopic}**, the most critical principle is understanding invariant enforcement at compile time.\n\nHere is a practical code example:\n\`\`\`rust\nuse std::sync::Arc;\nuse tokio::sync::Mutex;\n\n#[tokio::main]\nasync fn main() {\n    let counter = Arc::new(Mutex::new(0));\n    let mut handles = vec![];\n    for _ in 0..10 {\n        let c = Arc::clone(&counter);\n        handles.push(tokio::spawn(async move {\n            let mut num = c.lock().await;\n            *num += 1;\n        }));\n    }\n    for h in handles { h.await.unwrap(); }\n    println!("Result: {}", *counter.lock().await);\n}\n\`\`\`\n\nNotice how the lifetime is managed across asynchronous task boundaries. Would you like to run the Checkpoint Assessment now?`;
      }
      setMessages((prev) => [...prev, { role: 'assistant', content: reply }]);
    }, 600);
  };

  const triggerSampleEvaluation = () => {
    setAssessmentResult({
      status: 'NEEDS_REVISION',
      score: 72,
      gaps: ['Async lock reentrancy deadlock conditions', 'Non-lexical lifetime drop scope rules'],
      feedback:
        'You demonstrated solid conceptual understanding of mutexes and thread spawning, but practical exercises revealed vulnerability to async deadlocks across await points.',
    });
  };

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div className="flex items-center space-x-3">
          <div className="w-10 h-10 rounded-2xl bg-amber-500/10 border border-amber-500/20 flex items-center justify-center text-amber-400">
            <Bot className="w-5 h-5" />
          </div>
          <div>
            <h2 className="text-xl font-bold text-white flex items-center space-x-2">
              <span>Engineering Mentor</span>
              <span className="text-xs font-normal text-amber-400 bg-amber-500/10 px-2 py-0.5 rounded-full border border-amber-500/20">
                Active Topic: {currentTopic}
              </span>
            </h2>
            <p className="text-xs text-slate-400 mt-0.5">
              Personalized technical tutor adapted to your background and knowledge gaps.
            </p>
          </div>
        </div>

        <button
          onClick={triggerSampleEvaluation}
          className="flex items-center space-x-1.5 bg-[#1e232e] hover:bg-[#282f3e] text-slate-200 text-xs font-medium px-3.5 py-2 rounded-full border border-white/[0.08] transition-colors"
        >
          <Sparkles className="w-3.5 h-3.5 text-amber-400" />
          <span>Simulate Checkpoint Audit</span>
        </button>
      </div>

      {/* Assessment Audit Pill if active */}
      {assessmentResult && (
        <div className="rounded-2xl bg-[#161a23] border border-amber-500/30 p-4 space-y-3 shadow-glow-amber">
          <div className="flex items-center justify-between">
            <div className="flex items-center space-x-2.5">
              <span className="w-2.5 h-2.5 rounded-full bg-amber-400 animate-pulse" />
              <span className="text-xs font-bold text-white uppercase tracking-wider">
                Checkpoint Result: {assessmentResult.status} ({assessmentResult.score}%)
              </span>
            </div>
            <button
              onClick={() => setAssessmentResult(null)}
              className="text-xs text-slate-400 hover:text-white"
            >
              Dismiss
            </button>
          </div>

          <p className="text-xs text-slate-300">{assessmentResult.feedback}</p>

          <div className="space-y-1">
            <span className="text-[11px] font-semibold text-amber-400 uppercase">Identified Knowledge Gaps:</span>
            <ul className="text-xs text-slate-400 list-disc list-inside space-y-0.5">
              {assessmentResult.gaps.map((gap, i) => (
                <li key={i}>{gap}</li>
              ))}
            </ul>
          </div>
        </div>
      )}

      {/* Chat Window */}
      <div className="rounded-3xl bg-[#12151b] border border-white/[0.07] p-5 flex flex-col h-[520px] shadow-lg">
        {/* Messages */}
        <div className="flex-1 overflow-y-auto space-y-4 pr-2">
          {messages.map((msg, index) => {
            const isUser = msg.role === 'user';
            return (
              <div
                key={index}
                className={`flex items-start space-x-3 ${isUser ? 'flex-row-reverse space-x-reverse' : ''}`}
              >
                <div
                  className={`w-7 h-7 rounded-full flex items-center justify-center text-xs flex-shrink-0 ${
                    isUser ? 'bg-amber-500 text-black font-semibold' : 'bg-white/[0.08] text-slate-200'
                  }`}
                >
                  {isUser ? 'You' : <Bot className="w-3.5 h-3.5" />}
                </div>

                <div
                  className={`max-w-2xl rounded-2xl px-4 py-3 text-xs leading-relaxed whitespace-pre-wrap ${
                    isUser
                      ? 'bg-amber-500/10 border border-amber-500/20 text-amber-100'
                      : 'bg-[#181c25] border border-white/[0.06] text-slate-200'
                  }`}
                >
                  {msg.content}
                </div>
              </div>
            );
          })}
        </div>

        {/* Input Bar */}
        <div className="pt-4 border-t border-white/[0.06] flex items-center space-x-2">
          <input
            type="text"
            value={input}
            onChange={(e) => setInput(e.target.value)}
            onKeyDown={(e) => e.key === 'Enter' && handleSend()}
            placeholder="Ask a technical question or type 'start checkpoint'..."
            className="flex-1 bg-[#181c25] border border-white/[0.08] rounded-full px-4 py-2.5 text-xs text-slate-200 placeholder-slate-500 focus:outline-none focus:border-amber-400/50"
          />
          <button
            onClick={handleSend}
            className="w-9 h-9 rounded-full bg-amber-500 hover:bg-amber-400 text-black flex items-center justify-center transition-transform active:scale-95 shadow-md shadow-amber-500/20"
          >
            <Send className="w-4 h-4 ml-0.5" />
          </button>
        </div>
      </div>
    </div>
  );
};
