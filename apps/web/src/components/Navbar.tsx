import React from 'react';
import { Bell, ChevronDown, User as UserIcon } from 'lucide-react';
import { UserProfile } from '../types';

interface NavbarProps {
  activeTab: 'overview' | 'planner' | 'projects' | 'goals';
  onSelectTab: (tab: 'overview' | 'planner' | 'projects' | 'goals') => void;
  profile?: UserProfile;
}

export const Navbar: React.FC<NavbarProps> = ({
  activeTab,
  onSelectTab,
  profile,
}) => {
  const tabs: Array<{ id: 'overview' | 'planner' | 'projects' | 'goals'; label: string }> = [
    { id: 'overview', label: 'Overview' },
    { id: 'planner', label: 'Planner' },
    { id: 'projects', label: 'Projects' },
    { id: 'goals', label: 'Goals' },
  ];

  const displayName = profile?.display_name || 'Kate Schowalter';

  return (
    <header className="w-full bg-[#0c0e12]/80 backdrop-blur-md border-b border-white/[0.06] sticky top-0 z-50 px-6 py-3.5">
      <div className="max-w-7xl mx-auto flex items-center justify-between">
        {/* Left: Brand Icon + Navigation Links */}
        <div className="flex items-center space-x-8">
          {/* Stylized Brand Logo (matching mockup gold emblem) */}
          <div className="flex items-center space-x-3 cursor-pointer" onClick={() => onSelectTab('overview')}>
            <div className="w-8 h-8 rounded-full bg-gradient-to-br from-amber-400 to-amber-600 flex items-center justify-center shadow-lg shadow-amber-500/20">
              <span className="font-bold text-black text-sm tracking-tighter">W1</span>
            </div>
            <span className="font-bold text-white tracking-wide text-sm hidden sm:inline-block">WORKSPACE</span>
          </div>

          {/* Navigation Links */}
          <nav className="flex items-center space-x-7">
            {tabs.map((tab) => {
              const isActive = activeTab === tab.id;
              return (
                <button
                  key={tab.id}
                  onClick={() => onSelectTab(tab.id)}
                  className={`relative py-1 text-sm font-medium transition-colors ${
                    isActive ? 'text-white' : 'text-slate-400 hover:text-slate-200'
                  }`}
                >
                  {tab.label}
                  {isActive && (
                    <span className="absolute bottom-0 left-0 w-full h-[2px] bg-gradient-to-r from-amber-400 to-amber-500 rounded-full animate-fade-in" />
                  )}
                </button>
              );
            })}
          </nav>
        </div>

        {/* Right: Actions & User Dropdown */}
        <div className="flex items-center space-x-3">
          {/* Quick Profile Icon */}
          <button
            title="Account"
            className="w-9 h-9 rounded-full bg-white/[0.04] hover:bg-white/[0.08] border border-white/[0.06] flex items-center justify-center text-slate-300 transition-colors"
          >
            <UserIcon className="w-4 h-4" />
          </button>

          {/* Notifications Bell */}
          <button
            title="Notifications"
            className="w-9 h-9 rounded-full bg-white/[0.04] hover:bg-white/[0.08] border border-white/[0.06] flex items-center justify-center text-slate-300 transition-colors relative"
          >
            <Bell className="w-4 h-4" />
            <span className="absolute top-2 right-2 w-1.5 h-1.5 bg-amber-400 rounded-full" />
          </button>

          {/* User Profile Pill */}
          <div className="flex items-center space-x-2.5 bg-white/[0.04] hover:bg-white/[0.07] border border-white/[0.06] rounded-full pl-1.5 pr-3 py-1 cursor-pointer transition-colors">
            <div className="w-7 h-7 rounded-full bg-slate-700 overflow-hidden flex items-center justify-center text-xs font-semibold text-white">
              {profile?.avatar_url ? (
                <img src={profile.avatar_url} alt={displayName} className="w-full h-full object-cover" />
              ) : (
                <span>{displayName.slice(0, 2).toUpperCase()}</span>
              )}
            </div>
            <span className="text-xs font-medium text-slate-200">{displayName}</span>
            <ChevronDown className="w-3.5 h-3.5 text-slate-400" />
          </div>
        </div>
      </div>
    </header>
  );
};
